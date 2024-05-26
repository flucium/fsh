#[macro_use]
mod macros;
mod manifest;
mod profile;

use clap::Parser as _;
use fsh_common::ErrorKind;
use fsh_engine::{execute, ShVars, State};
use fsh_parser::Parser;
use fsh_terminal::Terminal;

/// The command-line arguments.
#[derive(Debug, clap::Parser)]
#[clap(name = manifest::MANIFEST_FSH_NAME, version = manifest::MANIFEST_FSH_VERSION, author = manifest::MANIFEST_FSH_AUTHORS)]
struct Args {
    #[clap(short = 'p', long = "profile")]
    profile: Option<String>,
}

/// Parses the command-line arguments.
///
/// # Returns
/// - 1 The profile path.
fn args() -> String {
    let args = Args::parse();

    args.profile
        .unwrap_or_else(|| profile::DEFAULT_PROFILE_PATH.to_string())
}

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
    // let mut terminal = Terminal::new();

    // terminal.set_prompt(sh_vars.get_prompt().unwrap_or_default());

    Terminal::from(sh_vars.get_prompt().unwrap_or_default())
}

/// Initializes the shell.
///
/// # Returns
/// - 1 The terminal.
/// - 2 The shell state.
/// - 3 The shell variables.
fn initialize() -> (Terminal, State, ShVars) {
    // Get the profile path.
    let profile_path = args();

    // Initialize the shell state.
    let mut state = init_engine_state();

    // Initialize the shell variables.
    let mut sh_vars = init_engine_sh_vars();

    // Initialize the shell profile.
    init_profile(&mut state, &mut sh_vars, profile_path);

    // Initialize the terminal.
    let terminal = init_terminal(&mut sh_vars);

    (terminal, state, sh_vars)
}

fn repl() {
    let (mut terminal, mut state, mut sh_vars) = initialize();

    loop {
        state
            .current_dir_mut()
            .push(sh_vars.get_cwd().unwrap_or_default());

        terminal.update_prompt(sh_vars.get_prompt().unwrap_or_default());

        match terminal.read_line() {
            Ok(input) => {
                if input.is_empty() {
                    continue;
                }

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
            Err(err) => {
                let msg = err.message();
                fsh_eprintln!("{msg}", err.kind());
            }
        }
    }
}

fn main() {
    repl();
}
