#[macro_use]
mod macros;
mod manifest;
mod profile;

use clap::Parser as _;
use fsh_common::ErrorKind;
use fsh_engine::{execute, ShVars, State};
use fsh_parser::Parser;
use fsh_terminal::Terminal;

/// Initializes the shell profile.
///
/// # Arguments
/// - `state` - The shell state.
/// - `sh_vars` - The shell variables.
///
/// # Panics
/// If an error occurs while reading or creating a profile, write to stderr if the cause of the error is known. Unexplained errors cause a panic.
fn init_profile(state: &mut State, sh_vars: &mut ShVars, path: impl Into<String>) {
    let path = &path.into();

    let profile_content = if profile::exists(path) {
        match profile::read(path) {
            Ok(string) => string,
            Err(err) => {
                let msg = err.message();

                if err.kind() == &ErrorKind::Other {
                    fsh_panic!("{msg}", err.kind());
                } else {
                    fsh_eprintln!("{msg}", err.kind());
                    String::new()
                }
            }
        }
    } else {
        match profile::create(path) {
            Ok(string) => string,
            Err(err) => {
                let msg = err.message();

                if err.kind() == &ErrorKind::Other {
                    fsh_panic!("{msg}", err.kind());
                } else {
                    fsh_eprintln!("{msg}", err.kind());
                    String::new()
                }
            }
        }
    };

    let mut parser = Parser::new(&profile_content);

    match parser.parse() {
        Ok(ast) => {
            if let Err(err) = execute(ast, state, sh_vars) {
                let msg = err.message();
                fsh_eprintln!("{msg}", err.kind());
            }
        }
        Err(err) => {
            let msg = err.message();
            fsh_eprintln!("{msg}", err.kind());
        }
    }
}

/// Initializes the shell state.
fn init_engine_state() -> State {
    let mut state = State::new();

    state.current_dir_mut().push("/");

    state
}

/// Initializes the shell variables.
fn init_engine_sh_vars() -> ShVars {
    let mut sh_vars = ShVars::new();

    sh_vars.inherit(std::env::vars());

    sh_vars
}

/// Initializes the terminal.
fn init_terminal(sh_vars: &mut ShVars) -> Terminal {
    let mut terminal = Terminal::new();

    terminal.set_prompt(sh_vars.get_prompt().unwrap_or_default());

    terminal
}

#[derive(Debug, clap::Parser)]
#[clap(name = manifest::MANIFEST_FSH_NAME, version = manifest::MANIFEST_FSH_VERSION, author = manifest::MANIFEST_FSH_AUTHORS)]
struct Args {
    #[clap(short = 'p', long = "profile")]
    profile: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path = args
        .profile
        .unwrap_or_else(|| profile::DEFAULT_PROFILE_PATH.to_string());

    let mut sh_vars = init_engine_sh_vars();

    let mut state = init_engine_state();

    let mut terminal = init_terminal(&mut sh_vars);

    init_profile(&mut state, &mut sh_vars, path);

    loop {
        state
            .current_dir_mut()
            .push(sh_vars.get_cwd().unwrap_or_default());

        terminal.set_prompt(sh_vars.get_prompt().unwrap_or_default());

        let input = terminal.read_line().unwrap();

        let mut parser = Parser::new(&input);

        match parser.parse() {
            Ok(ast) => {
                if let Err(err) = execute(ast, &mut state, &mut sh_vars) {
                    let msg = err.message();
                    fsh_eprintln!("{msg}", err.kind());
                }
            }
            Err(err) => {
                let msg = err.message();
                fsh_eprintln!("{msg}", err.kind());
            }
        }
    }
}
