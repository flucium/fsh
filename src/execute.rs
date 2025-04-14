use crate::{
    ast::{expression::*, statement::*},
    builtin,
    error::*,
    pipe::{self, PipeState},
    result::Result,
    sh_vars::ShVars,
    state::State,
    utils::globbing,
};

use std::{
    fs, io,
    os::fd::{AsRawFd, FromRawFd, IntoRawFd},
    process,
};

use std::os::unix::process::CommandExt;

/// Executes an assignment by updating the shell variables.
///
/// This function takes an `Assignment` and evaluates its identifier and value.
/// If the identifier is valid and the value is a supported expression type,
/// the key-value pair is inserted into the provided `ShVars` instance.
///
/// # Arguments
/// - `assignment`: The `Assignment` to be executed.
/// - `sh_vars`: A mutable reference to the shell variable store.
///
/// # Returns
/// - `Ok(())`: If the assignment is valid and successfully applied.
/// - `Err(Error::NOT_IMPLEMENTED)`: If the identifier is not an `Expression::Identifier`
///   or the value is of an unsupported expression type.
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

/// Executes a built-in shell command.
///
/// This function matches the given command name against supported built-in commands
/// and executes the corresponding operation. Supported commands include:
/// - `cd`: Changes the current working directory.
/// - `abort`: Aborts the process immediately.
/// - `exit`: Exits the process with the specified exit code.
///
/// # Arguments
/// - `name`: The name of the built-in command.
/// - `args`: A vector of string arguments passed to the command.
/// - `state`: A mutable reference to the shell state, used for operations like changing directories.
///
/// # Returns
/// - `Ok(())`: If the command is executed successfully.
/// - `Err(Error::NOT_IMPLEMENTED)`: If the command is not a supported built-in.
fn execute_builtin_command(name: &String, args: &Vec<String>, state: &mut State) -> Result<()> {
    match name.as_str() {
        "cd" => {
            builtin::cd(
                args.get(0).unwrap_or(&String::from("/")),
                state.current_dir(),
            )?;
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

/// Executes an external process command.
///
/// This function constructs and configures a new `std::process::Command`
/// using the provided name, arguments, redirections, and execution context.
/// It handles standard I/O setup (including pipe redirection), applies
/// redirection rules via `dup2`, and spawns the process. If the process is
/// not intended to run in the background, it wires up its output appropriately.
///
/// # Arguments
/// - `name`: The name of the executable.
/// - `args`: Arguments passed to the executable.
/// - `redirects`: List of redirection specifications.
/// - `is_background`: Indicates whether the process should run in the background.
/// - `state`: Mutable reference to shell state (for managing pipes and handlers).
/// - `sh_vars`: Mutable reference to shell variables (used for variable lookup).
/// - `is_last`: Whether this command is the last in a pipeline.
///
/// # Returns
/// - `Ok(())` if the process is successfully spawned.
/// - `Err(io::Error)` if there is any error during process construction or execution.
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

/// Executes a command (built-in or external).
///
/// This function resolves the command name and arguments from the `Command` AST node,
/// handles glob expansion, resolves identifiers using shell variables,
/// and attempts to execute it as a built-in or external command.
///
/// # Arguments
/// - `command`: The parsed `Command` to execute.
/// - `state`: Mutable reference to shell state.
/// - `sh_vars`: Mutable reference to shell variables.
/// - `is_last`: Whether this is the last command in a pipeline.
///
/// # Returns
/// - `Ok(())` if the command is executed successfully.
/// - `Err(Error::NOT_IMPLEMENTED)` if resolution or execution fails.
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


/// Executes a top-level AST statement recursively.
///
/// This function dispatches execution based on the type of `Statement`,
/// handling sequences, assignments, commands, redirections (TODO), and pipelines.
///
/// # Arguments
/// - `ast`: The `Statement` AST node to execute.
/// - `state`: Mutable reference to shell state.
/// - `sh_vars`: Mutable reference to shell variables.
///
/// # Returns
/// - `Ok(())` if the statement executes successfully.
/// - `Err(Error)` if any execution error occurs.
pub fn execute(ast: Statement, state: &mut State, sh_vars: &mut ShVars) -> Result<()> {
    match ast {
        Statement::Sequence(mut sequence) => {
            while let Some(ast) = sequence.pop_front() {
                execute(ast, state, sh_vars)?;
            }
        }

        Statement::Assignment(assignment) => {
            execute_assignment(assignment, sh_vars)?;
        }

        // ToDo
        Statement::Redirect(_) => todo!(),

        Statement::Command(command) => execute_command(command, state, sh_vars, true)?,

        Statement::Pipe(mut pipe) => {
            *state.pipe_mut() = pipe::Pipe::open();

            while let Some(command) = pipe.pop_front() {
                execute_command(command, state, sh_vars, pipe.is_empty())?;
            }

            state.pipe_mut().close()?;
            state.handler_mut().wait();
        }
    }

    Ok(())
}
