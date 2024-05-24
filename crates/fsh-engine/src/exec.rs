use fsh_common::{Error, ErrorKind, Result};

use std::fs;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::process::CommandExt;
use std::process;

use super::{extract::*, pipe::Pipe, ShVars, State};

/// Execute the AST.
pub fn execute(ast: fsh_ast::Ast, state: &mut State, sh_vars: &mut ShVars) -> Result<()> {
    match ast {
        fsh_ast::Ast::Block(mut block) => {
            while let Some(ast) = block.pop_front() {
                execute(ast, state, sh_vars)?;
            }
        }
        fsh_ast::Ast::Pipe(mut pipe) => {
            *state.pipe_mut() = Pipe::open();

            while let Some(command) = pipe.pop_front() {
                execute_command(command, state, sh_vars, pipe.is_empty())?;
            }

            state.pipe_mut().close()?;

            state.handler_mut().wait();
        }
        fsh_ast::Ast::Statement(statement) => match statement {
            fsh_ast::Statement::Command(command) => {
                execute_command(command, state, sh_vars, true)?;

                state.handler_mut().wait();
            }
            fsh_ast::Statement::Assign(assign) => {
                execute_assign(assign, sh_vars)?;
            }
        },
    }

    Ok(())
}

/// Execute the command.
fn execute_command(
    command: fsh_ast::Command,
    state: &mut State,
    sh_vars: &mut ShVars,
    is_last: bool,
) -> Result<()> {
    let name = extract_command_name(&command, sh_vars)?;

    let args = extract_command_args(&command, sh_vars)?;

    let redirects = command.redirects;

    execute_builtin_command(&name, &args, state).or(execute_process_command(
        &name,
        &args,
        redirects,
        command.background,
        state,
        sh_vars,
        is_last,
    ))?;

    Ok(())
}

/// Execute the builtin command.
fn execute_builtin_command(name: &String, args: &Vec<String>, state: &mut State) -> Result<()> {
    match name.as_str() {
        "abort" => super::builtin::common::abort(),

        "cd" => super::builtin::unix::cd(args.first().unwrap_or(&String::from("/")), state)?,

        "exit" => {
            super::builtin::common::exit(args.first().map_or(0, |code| code.parse().unwrap_or(0)))
        }

        _ => Err(Error::new(
            ErrorKind::NotFound,
            &format!("{name}: command not found"),
        ))?,
    }

    Ok(())
}

/// Execute the process command.
fn execute_process_command(
    name: &String,
    args: &Vec<String>,
    redirects: Vec<fsh_ast::Redirect>,
    is_background: bool,
    state: &mut State,
    sh_vars: &mut ShVars,
    is_last: bool,
) -> Result<()> {
    let mut ps_command = process::Command::new(name);

    ps_command.args(args);

    ps_command.stdin(if state.pipe().is_recvable() {
        unsafe { process::Stdio::from_raw_fd(state.pipe_mut().recv().unwrap()) }
    } else {
        process::Stdio::inherit()
    });

    ps_command.stdout(if state.pipe().is_sendable() {
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
        let sh_vars_clone = sh_vars.clone();

        ps_command.pre_exec(move || {
            for redirect in &redirects {
                let left = match redirect.left {
                    fsh_ast::Expr::FD(fd) => fd,

                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "invalid file descriptor",
                    ))?,
                };

                let right = match &redirect.right {
                    fsh_ast::Expr::String(string) => {
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

                    fsh_ast::Expr::Ident(ident) => {
                        let string = match sh_vars_clone.get(&ident) {
                            Ok(val) => val,
                            Err(_) => Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "invalid file descriptor",
                            ))?,
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

                    fsh_ast::Expr::Number(number) => {
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

                    fsh_ast::Expr::FD(fd) => *fd,
                };

                match redirect.operator {
                    fsh_ast::RedirectOperator::Gt => {
                        libc::dup2(right, left);
                    }
                    fsh_ast::RedirectOperator::Lt => {
                        libc::dup2(right, left);
                    }
                }
            }

            Ok(())
        });
    };

    // spawn the process
    let child = ps_command.spawn().map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            Error::new(ErrorKind::NotFound, &format!("{name}: command not found"))
        } else {
            let err_msg = err.to_string();
            Error::new(ErrorKind::Other, &err_msg)
        }
    })?;

    // push the process to the handler
    let pid = state.handler_mut().push(child, is_background);

    // send the stdout to the pipe
    if let Some(child) = state.handler().get(pid) {
        if let Some(stdout) = child.stdout.as_ref() {
            let fd = stdout.as_raw_fd();
            state.pipe_mut().send(fd).unwrap();
        }
    }

    Ok(())
}

/// Execute the assign.
fn execute_assign(assign: fsh_ast::Assign, sh_vars: &mut ShVars) -> Result<()> {
    let assign = extract_assign(assign)?;

    sh_vars.insert(assign.0, assign.1);

    Ok(())
}
