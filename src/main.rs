use std::env;

use fsh::{
    execute::execute, parser::Parser, preprocessor::preprocess, sh_vars::ShVars, state::State,
    terminal::Terminal,
};

fn main() {
    // println!(
    //     "{:?}",
    //     Parser::new(preprocess("ls -a\n\nping -c 3 flucium.net")).parse()
    // );

    let mut vars = ShVars::new();
    vars.inherit(env::vars());

    let mut state = State::new();
    state.current_dir_mut().push(env::current_dir().unwrap());

    let mut terminal = Terminal::new();

    loop {
        let string = terminal.read_line().unwrap();

        if let Err(err) = execute(
            Parser::new(preprocess(string)).parse().unwrap(),
            &mut state,
            &mut vars,
        ) {
            println!("fsh: {}", err.message());
        }
    }
}
