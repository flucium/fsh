use std::{
    env,
    io::{self, Write},
};

use clap::Parser;
use fsh::profile;

#[derive(Parser)]
struct AppArgs {
    #[clap(long="profile", short='p', default_value = profile::DEFAULT_PROFILE_PATH)]
    profile: String,
}

fn parse_app_args() -> String {
    let args = AppArgs::parse();

    args.profile
}

fn initialize() -> (fsh::state::State, fsh::sh_vars::ShVars) {
    let path = parse_app_args();

    let profile_content = profile::exists(&path)
        .then(|| profile::read_profile(&path))
        .map_or_else(
            || {
                let content = profile::DEFAULT_PROFILE;
                profile::write_profile(&path, content)
                    .map_err(|_| {
                        // eprintln!("Error: {}", e);
                        std::process::exit(1);
                    })
                    .unwrap();

                content.to_string()
            },
            |result| {
                result
                    .map_err(|_| {
                        // eprintln!("Error: {}", e);
                        std::process::exit(1);
                    })
                    .unwrap()
            },
        );

    // Initialize the shell state
    let mut state = fsh::state::State::new();

    state
        .current_dir_mut()
        .push(env::current_dir().unwrap_or_default());

    // Initialize the shell variables
    let mut sh_vars = fsh::sh_vars::ShVars::from(env::vars());

    // Parse the profile content
    if let Err(err) = fsh::parser::Parser::new(profile_content)
        .parse()
        .map(|ast| fsh::execute::execute(ast, &mut state, &mut sh_vars))
    {
        if *err.kind() == fsh::error::ErrorKind::InvalidSyntax {
            stderr(format!("fsh: {}\n", err).as_str());
        } else {
            stderr(format!("fsh: {}\n", err).as_str());
        }

        std::process::exit(1);
    }

    (state, sh_vars)
}

fn stderr(msg: &str) {
    if io::stderr().write_all(msg.as_bytes()).is_err() {
        panic!("failed to write to stderr");
    }
}

fn main() {
    let (mut state, mut sh_vars) = initialize();

    let mut terminal = fsh::terminal::Terminal::new();

    while let Ok(string) = terminal.read_line() {
        let mut parser = fsh::parser::Parser::new(string);

        match parser.parse() {
            Ok(ast) => {
                if let Err(err) = fsh::execute::execute(ast, &mut state, &mut sh_vars) {
                    if *err.kind() == fsh::error::ErrorKind::NotFound {
                        stderr(format!("fsh: command not found: {}\n", err).as_str());
                    } else {
                        stderr(format!("fsh: {}\n", err).as_str());
                    }
                }
            }
            Err(err) => {
                if *err.kind() == fsh::error::ErrorKind::InvalidSyntax {
                    continue;
                } else {
                    stderr(format!("fsh: {}\n", err).as_str());
                }
            }
        }
    }
}
