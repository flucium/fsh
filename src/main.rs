use std::{
    env,
    io::{self, Write},
    process,
};

use fsh::{
    execute::execute,
    parser::Parser,
    profile::{self, DEFAULT_PROFILE_CONTENT},
    prompt,
    sh_vars::ShVars,
    state::State,
    terminal::Terminal,
};

use clap;

fn stdout(message: impl Into<String>) -> io::Result<()> {
    io::stdout().write_all(message.into().as_bytes())
}

fn stderr(msg: &str) {
    if io::stderr().write_all(msg.as_bytes()).is_err() {
        panic!("failed to write to stderr");
    }
}

fn welcome_art() {
    let art = r#"
+---------------------------------+
|                                 |
| >_                              |
|                                 |
| Welcome to FSH                  |
|                                 |
| https://github.com/flucium/fsh  |
|                                 |
+---------------------------------+
"#;

    if stdout(art).is_err() {
        stderr("failed to write to stdout");
    }
}

#[derive(clap::Parser)]
struct AppArgs {
    #[clap(long="profile", short='p', default_value = profile::DEFAULT_PROFILE_PATH)]
    profile: String,
}

fn parse_app_args() -> String {
    let args = <AppArgs as clap::Parser>::parse();

    args.profile
}

fn initialize() -> (State, ShVars) {
    let path = parse_app_args();

    let profile_content = profile::exists(&path)
        .then(|| profile::read_profile(&path))
        .map_or_else(
            || {
                profile::write_profile(&path, DEFAULT_PROFILE_CONTENT)
                    .map_err(|_| {
                        process::exit(1);
                    })
                    .unwrap();

                DEFAULT_PROFILE_CONTENT.to_string()
            },
            |result| {
                result
                    .map_err(|_| {
                        process::exit(1);
                    })
                    .unwrap()
            },
        );

    let mut state = State::new();

    state
        .current_dir_mut()
        .push(env::current_dir().unwrap_or_default());

    let mut sh_vars = ShVars::from(env::vars());

    if let Err(err) = Parser::new(profile_content)
        .parse()
        .map(|ast| execute(ast, &mut state, &mut sh_vars))
    {
        // if *err.kind() == fsh::error::ErrorKind::InvalidSyntax {
        //     stderr(format!("fsh: {}\n", err).as_str());
        // } else {
        //     stderr(format!("fsh: {}\n", err).as_str());
        // }

        stderr(format!("fsh: {}\n", err).as_str());

        process::exit(1);
    }

    (state, sh_vars)
}

fn main() {
    welcome_art();

    let (mut state, mut sh_vars) = initialize();

    let mut terminal = Terminal::new();

    terminal.set_prompt(prompt::decode(
        sh_vars.get("FSH_PROMPT").map_or("> ", |string| string),
    ));

    while let Ok(string) = terminal.read_line() {
        let mut parser = Parser::new(string);

        match parser.parse() {
            Ok(ast) => {
                if let Err(err) = execute(ast, &mut state, &mut sh_vars) {
                    // if *err.kind() == fsh::error::ErrorKind::NotFound {
                    //     stderr(format!("fsh: command not found: {}\n", err).as_str());
                    // } else {
                    //     stderr(format!("fsh: {}\n", err).as_str());
                    // }
                    stderr(format!("fsh: {}\n", err).as_str());
                }
            }
            Err(err) => {
                // if *err.kind() == fsh::error::ErrorKind::InvalidSyntax {
                //     continue;
                // } else {
                //     stderr(format!("fsh: {}\n", err).as_str());
                // }
                stderr(format!("fsh: {}\n", err).as_str());
            }
        }
    }
}
