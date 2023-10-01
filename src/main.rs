use clap::Parser;
use lets_get_coding::lib::{lgc_commands, python, julia};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[clap(value_enum)]
    language: lgc_commands::Language,
    name: String,
}

fn main() {

    let args = Cli::parse();
    let language = &args.language;
    let project_name = args.name.replace(" ", "_").replace("-", "_");

    // very early stages of sussing out code in main, starting with python
    match language {
        lgc_commands::Language::Python => {
            python::build_env(&project_name).expect("Failed to build Python environment")
        },
        lgc_commands::Language::Julia => {
            julia::build_env(&project_name).expect("Failed to build environment")
        },
        _ => panic!("Other languages are not yet supported."),
    }
}
