use std::{fs, io, path::PathBuf, process};

use std::os::unix::{io::IntoRawFd, process::CommandExt};

use crate::{
    ast::{expression::*, statement::*},
    builtin,
    error::{Error, ErrorKind},
    result::Result,
    sh_vars::ShVars,
    state::State,
    utils::path::PathBufExt,
};

fn execute_assignment(assignment: Assignment, sh_vars: &mut ShVars) -> Result<()> {
    let identifier = match assignment.identifier() {
        Expression::Identifier(identifier) => identifier.to_string(),
        _ => Err(Error::new(
            ErrorKind::ExecutionFailed,
            "invalid identifier expression",
        ))?,
    };

    let value = match assignment.value() {
        Expression::Null => String::default(),
        Expression::String(string) => string.to_string(),
        Expression::Boolean(boolean) => boolean.to_string(),
        Expression::Number(number) => number.to_string(),
        Expression::FileDescriptor(filedescriptor) => filedescriptor.to_string(),
        _ => Err(Error::new(
            ErrorKind::ExecutionFailed,
            "invalid assignment value expression",
        ))?,
    };

    sh_vars.insert(identifier, value)
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

        _ => Err(Error::new(
            ErrorKind::NotFound,
            format!("{name}: command not found"),
        ))?,
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
) -> Result<()> {
    let mut ps_command = process::Command::new(&name);

    ps_command.args(args);

    if let Some(ref pipe) = state.pipe().0 {
        ps_command.stdin(pipe.try_clone().unwrap());
    } else {
        ps_command.stdin(process::Stdio::inherit());
    }

    if let Some(ref pipe) = state.pipe().1 {
        ps_command.stdout(pipe.try_clone().unwrap());
    } else {
        ps_command.stdout(process::Stdio::inherit());
    }

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

                    _ => Err(Error::new(
                        ErrorKind::InvalidFileDescriptor,
                        "invalid left-hand side of redirect",
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
                            Err(e) => Err(e)?,
                        }
                    }
                    &Expression::Identifier(ref identifier) => {
                        let string = match sh_vars_cloned.get(identifier) {
                            None => Err(Error::new(
                                ErrorKind::NotFound,
                                "redirect target not found in environment",
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
                            Err(e) => Err(e)?,
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
                            Err(e) => Err(e)?,
                        }
                    }

                    &Expression::FileDescriptor(fd) => fd,
                    _ => Err(Error::new(
                        ErrorKind::InvalidFileDescriptor,
                        "invalid right-hand side of redirect",
                    ))?,
                };

                match redirect.operator() {
                    RedirectOperator::GreaterThan => {
                        redirection(right, left)?;
                    }
                    RedirectOperator::LessThan => {
                        redirection(right, left)?;
                    }
                }
            }

            Ok(())
        });
    }
    
    let child = ps_command.spawn().map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => {
            Error::new(ErrorKind::NotFound, format!("{name}: command not found"))
        }
        _ => Error::new(
            ErrorKind::ExecutionFailed,
            format!("{name}: command failed to start"),
        ),
    })?;

    state.processes_mut().push((child, is_background));

    // if let Some(child) = state.handler().get(pid) {
    //     child
    // }

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
            None => Err(Error::new(
                ErrorKind::NotFound,
                "command not found in environment",
            ))?,
            Some(string) => string,
        }
        .to_string(),

        Expression::Number(number) => number.to_string(),

        _ => Err(Error::new(
            ErrorKind::ExecutionFailed,
            "invalid command name expression",
        ))?,
    };

    let mut arguments = Vec::with_capacity(command.arguments().len());

    for argument in command.arguments() {
        let argument = match &argument {
            Expression::String(string) => {
                let mut string_vec = PathBuf::from(string)
                    .glob()?
                    .map(|path| path.unwrap_or_default().to_string_lossy().to_string())
                    .collect::<Vec<String>>();

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

            _ => Err(Error::new(
                ErrorKind::ExecutionFailed,
                "invalid command argument expression",
            ))?,
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

        Statement::Redirect(_) => {
            todo!()
        }

        Statement::Command(command) => {
            execute_command(command, state, sh_vars, true)?;

            if let Some(mut ps) = state.processes_mut().pop() {
                if ps.1 == false {
                    ps.0.wait().unwrap();
                }
            }
        }

        Statement::Pipe(mut pipe) => {
            let mut prev_r = None;

            while let Some(command) = pipe.pop_front() {
                let is_last = pipe.is_empty();

                let (r, mut w) = if is_last {
                    (None, None)
                } else {
                    let (r, w) = std::io::pipe()
                        .map_err(|_| Error::new(ErrorKind::Interrupted, "failed to create pipe"))?;
                    (Some(r), Some(w))
                };

                state.pipe_mut().0 = prev_r.take();
                state.pipe_mut().1 = w.take();

                execute_command(command, state, sh_vars, pipe.is_empty())?;

                prev_r = r;

                drop(w);
            }

            *state.pipe_mut() = (None, None);

            while let Some(mut ps) = state.processes_mut().pop() {
                if ps.1 == false {
                    ps.0.wait().unwrap();
                }
            }
        }
    }

    Ok(())
}

#[inline]
fn redirection(left: i32, right: i32) -> Result<()> {
    unsafe {
        if libc::dup2(left, right) >= 0 {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidFileDescriptor,
                "failed to duplicate file descriptor",
            ))
        }
    }
}
