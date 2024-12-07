use std::{
    fs, io,
    os::unix::{
        io::{AsRawFd, FromRawFd, IntoRawFd},
        process::CommandExt,
    },
    process,
};

use crate::{
    ast::{
        expression::Expression,
        statement::{Assignment, Command, Redirect, RedirectOperator, Statement},
        Node,
    },
    builtin,
    error::Error,
    pipe::{Pipe, PipeState},
    result::Result,
    sh_vars::ShVars,
    state::State,
    utils::globbing,
};

/// Executes an assignment statement.
///
/// This function evaluates an assignment statement, extracts the variable name and value,
/// and stores it in the shell variable environment.
///
/// # Arguments
/// - `assignment`: An `Assignment` struct representing the assignment statement.
/// - `sh_vars`: A mutable reference to the shell variable environment.
fn execute_assignment(assignment: Assignment, sh_vars: &mut ShVars) -> Result<()> {
    let identifier = match assignment.identifier() {
        Expression::Identifier(identifier) => identifier.to_string(),
        _ => Err(Error::NOT_IMPLEMENTED)?,
    };

    let value = match assignment.value() {
        Expression::Null => String::default(),
        Expression::String(string) => string.to_string(),
        Expression::Boolean(boolean) => boolean.to_string(),
        Expression::Number(number) => number.to_string(),
        Expression::FileDescriptor(filedescriptor) => filedescriptor.to_string(),
        _ => Err(Error::NOT_IMPLEMENTED)?,
    };

    sh_vars.insert(identifier, value);

    Ok(())
}

/// Executes a built-in command.
///
/// This function matches the command name with the available built-in commands
/// (e.g., `cd`, `exit`, `abort`) and performs the associated action.
///
/// # Arguments
/// - `name`: The name of the command to execute.
/// - `args`: A vector of arguments for the command.
/// - `state`: A mutable reference to the shell state.
fn execute_builtin_command(name: &String, args: &Vec<String>, state: &mut State) -> Result<()> {
    match name.as_str() {
        "cd" => {
            builtin::cd(args.get(0).unwrap_or(&String::from("/")), state)?;
        }

        "abort" => {
            builtin::abort();
        }

        "exit" => {
            builtin::exit(
                args.get(0)
                    .unwrap_or(&String::from("0"))
                    .parse::<i32>()
                    .unwrap_or(0),
            );
        }

        _ => Err(Error::NOT_IMPLEMENTED)?,
    }

    Ok(())
}

/// Executes a process command with optional redirections and background handling.
///
/// This function spawns a new process using the specified command and arguments.
/// It handles input/output redirection, environment variable setup, and background execution.
///
/// # Arguments
/// - `name`: The name of the executable.
/// - `args`: A vector of arguments to pass to the executable.
/// - `redirects`: A vector of `Redirect` objects defining input/output redirections.
/// - `is_background`: A boolean indicating if the process should run in the background.
/// - `state`: A mutable reference to the shell state.
/// - `sh_vars`: A mutable reference to the shell variable environment.
/// - `is_last`: A boolean indicating if this is the last command in a pipeline.
fn execute_process_command(
    name: String,
    args: Vec<String>,
    redirects: Vec<Redirect>,
    is_background: bool,
    state: &mut State,
    sh_vars: &mut ShVars,
    is_last: bool,
) -> io::Result<()> {
    let mut ps_command = process::Command::new(name);

    ps_command.args(args);

    ps_command.stdin(if state.pipe().state() == &PipeState::Recvable {
        unsafe { process::Stdio::from_raw_fd(state.pipe_mut().recv().unwrap()) }
    } else {
        process::Stdio::inherit()
    });

    ps_command.stdout(if state.pipe().state() == &PipeState::Sendable {
        process::Stdio::piped()
    } else {
        process::Stdio::inherit()
    });

    ps_command.stderr(process::Stdio::inherit());

    ps_command.envs(sh_vars.entries());

    ps_command.current_dir(state.current_dir());

    if is_last {
        ps_command.stdout(process::Stdio::inherit());
    }

    unsafe {
        let sh_vars_cloned = sh_vars.clone();
        ps_command.pre_exec(move || {
            for redirect in &redirects {
                let left = match redirect.left() {
                    Expression::FileDescriptor(fd) => *fd,

                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "invalid file descriptor",
                    ))?,
                };

                let right = match redirect.right() {
                    &Expression::String(ref string) => {
                        match fs::File::options()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open(string)
                        {
                            Ok(file) => file.into_raw_fd(),
                            Err(_) => Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "invalid file descriptor",
                            ))?,
                        }
                    }
                    &Expression::Identifier(ref identifier) => {
                        let string = match sh_vars_cloned.get(identifier) {
                            None => Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "invalid file descriptor",
                            ))?,
                            Some(string) => string,
                        };

                        match fs::File::options()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open(string)
                        {
                            Ok(file) => file.into_raw_fd(),
                            Err(_) => Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "invalid file descriptor",
                            ))?,
                        }
                    }

                    &Expression::Number(number) => {
                        match fs::File::options()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open(number.to_string())
                        {
                            Ok(file) => file.into_raw_fd(),
                            Err(_) => Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "invalid file descriptor",
                            ))?,
                        }
                    }

                    &Expression::FileDescriptor(fd) => fd,
                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "invalid file path",
                    ))?,
                };

                match redirect.operator() {
                    RedirectOperator::GreaterThan => {
                        libc::dup2(right, left);
                    }
                    RedirectOperator::LessThan => {
                        libc::dup2(right, left);
                    }
                }
            }

            Ok(())
        });
    }

    let child = ps_command.spawn()?;

    let pid = state.handler_mut().push(child, is_background);

    if let Some(child) = state.handler().get(pid) {
        if let Some(stdout) = child.stdout.as_ref() {
            let fd = stdout.as_raw_fd();
            state.pipe_mut().send(fd).unwrap();
        }
    }

    Ok(())
}

/// Executes a command statement, either built-in or as an external process.
///
/// This function first attempts to execute the command as a built-in. If the command
/// is not recognized as a built-in, it falls back to executing it as an external process.
///
/// # Arguments
/// - `command`: A `Command` struct representing the command to execute.
/// - `state`: A mutable reference to the shell state.
/// - `sh_vars`: A mutable reference to the shell variable environment.
/// - `is_last`: A boolean indicating if this is the last command in a pipeline.
fn execute_command(
    command: Command,
    state: &mut State,
    sh_vars: &mut ShVars,
    is_last: bool,
) -> Result<()> {
    let name = match command.name() {
        Expression::String(string) => string.to_owned(),

        Expression::Identifier(identifier) => match sh_vars.get(identifier) {
            None => Err(Error::NOT_IMPLEMENTED)?,
            Some(string) => string,
        }
        .to_string(),

        Expression::Number(number) => number.to_string(),

        _ => Err(Error::NOT_IMPLEMENTED)?,
    };

    let mut arguments = Vec::with_capacity(command.arguments().len());

    for argument in command.arguments() {
        let argument = match &argument {
            Expression::String(string) => {
                let mut string_vec = globbing(&string);
                if string_vec.len() > 0 {
                    arguments.append(&mut string_vec);

                    continue;
                } else {
                    string
                }
            }
            Expression::Number(number) => &number.to_owned().to_string(),
            Expression::Identifier(identifier) => &sh_vars
                .get(identifier)
                .unwrap_or(&String::default())
                .to_string(),
            _ => Err(Error::NOT_IMPLEMENTED)?,
        };

        arguments.push(argument.to_string());
    }

    let redirects = command.redirects().to_vec();

    let is_background = match command.is_background() {
        &Expression::Boolean(boolean) => boolean,
        _ => false,
    };

    execute_builtin_command(&name, &arguments, state).or_else(|_| {
        execute_process_command(
            name,
            arguments,
            redirects,
            is_background,
            state,
            sh_vars,
            is_last,
        )
        .map_err(|_| Error::NOT_IMPLEMENTED)
    })
}

/// Executes a shell abstract syntax tree (AST).
///
/// This function traverses the AST and executes its nodes, which may include blocks,
/// statements, commands, and pipelines. Each node is evaluated in sequence.
///
/// # Arguments
/// - `ast`: A `Node` representing the root of the AST.
/// - `state`: A mutable reference to the shell state.
/// - `sh_vars`: A mutable reference to the shell variable environment.
pub fn execute(ast: Node, state: &mut State, sh_vars: &mut ShVars) -> Result<()> {
    match ast {
        Node::Block(mut block) => {
            while let Some(ast) = block.pop() {
                execute(ast, state, sh_vars)?;
            }
        }

        Node::Statement(statement) => match statement {
            Statement::Command(command) => {
                execute_command(command, state, sh_vars, true)?;

                state.handler_mut().wait();
            }

            Statement::Assignment(assignment) => {
                execute_assignment(assignment, sh_vars)?;
            }
        },

        Node::Pipe(mut pipe) => {
            *state.pipe_mut() = Pipe::open();

            while let Some(command) = pipe.pop() {
                execute_command(command, state, sh_vars, pipe.is_empty())?;
            }

            state.pipe_mut().close()?;

            state.handler_mut().wait();
        }
    }

    Ok(())
}
