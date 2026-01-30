use clap::Parser;
use clap::{Arg, ArgAction, Command, ValueHint, value_parser};
use clap_complete::aot::{Generator, Shell, generate};
use std::io;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// some input file
    #[arg(value_hint = ValueHint::AnyPath)]
    file: Option<String>,

    /// Generate shell completion script for specified shell
    #[arg(long, action = ArgAction::Set, value_parser = value_parser!(Shell))]
    generator: Option<Shell>,
}
impl Cli {
    pub fn command() -> Command {
        Command::new("auto_completion")
            .about("An example application to demonstrate shell auto-completion generation using clap and clap_complete.")
            .arg(
                Arg::new("file")
                    .help("some input file")
                    .value_hint(ValueHint::AnyPath)
                    .required(false),
            )
            .arg(
                Arg::new("generator")
                    .long("generator")
                    .help("Generate shell completion script for specified shell")
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(Shell))
                    .required(false),
            )
    }
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn main() {
    let matches = Cli::parse();
    let mut cmd = Cli::command();
    if let Some(generator) = matches.generator {
        print_completions(generator, &mut cmd);
    }
}
