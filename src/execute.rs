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
            
            while let Some(command) = pipe.pop_front(){
                execute_command(command, state, sh_vars, pipe.is_empty())?;
            }

            state.pipe_mut().close()?;
            state.handler_mut().wait();
        },
    }

    Ok(())
}
