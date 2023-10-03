use clap::Parser;
use lets_get_coding::lib::*;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[clap(value_enum)]
    language: Language,
    name: String,
}

fn main() {
    let args = Cli::parse();
    let language = &args.language;
    let project_name = args.name.replace(" ", "_").replace("-", "_");

    // very early stages of sussing out code in main, starting with python
    match language {
        Language::Python => {
            python::build_env(&project_name).expect("Failed to build Python environment");
        }
        Language::Julia => {
            julia::build_env(&project_name).expect("Failed to build Julia environment");
        }
        Language::Go => {
            golang::build_env(&project_name).expect("Failed to build Go environment");
        }
        _ => panic!("Only Python, Julia, and Go are supported at this time."),
    }
}
