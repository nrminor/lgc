use clap::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::str::FromStr;

#[derive(clap::ValueEnum, Clone)]
pub enum Language {
    Python,
    Mojo,
    Julia,
    R,
    Go,
    Rust,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" => Ok(Language::Python),
            "mojo" => Ok(Language::Mojo),
            "julia" => Ok(Language::Julia),
            "r" => Ok(Language::R),
            "go" => Ok(Language::Go),
            "rust" => Ok(Language::Rust),
            _ => Err(()),
        }
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[clap(value_enum)]
    language: Language,
    name: String,
}

fn copy_template(project_name: &String) -> std::io::Result<()> {
    // Get the current executable's path
    let exe_path = env::current_exe()?;
    let root_dir = exe_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // Form the path to the source file relative to the executable
    let src_file = root_dir.join("templates/Python/template.py");

    // Get the current working directory
    let dest_dir = env::current_dir()?;

    // Form the path to the destination file
    let dest_file = dest_dir.join(format!("{}/{}/template.py", project_name, project_name));

    // Copy the file
    fs::copy(src_file, dest_file)?;

    Ok(())
}

fn main() {
    /*
    Sussing out basic structure of the program before using clap &c."). I'll start
    with setting up Python with Poetry. To do so, the program will need to go through
    the following steps:
    1 - Collect the project name and python version as command line arguments
    2 - Check if Poetry is installed, and if not, prompt the user to install it.
    3 - Run Poetry to set up the python environment.

    While complex now, main.rs will eventually be very simple and easy to read. The
    task is to suss out what I'll need here and then abstract away the complexity into
    lib crates.
    */

    let args = Cli::parse();
    let language = &args.language;
    let desired_name = args.name;

    let project_name = desired_name.replace(" ", "_").replace("-", "_");

    // very early stages of sussing out code in main, starting with python
    match language {
        Language::Python => {
            let check_poetry = Command::new("poetry").arg("--version").output();

            match check_poetry {
                Ok(_) => {
                    let _ = Command::new("poetry")
                        .arg("new")
                        .arg(&project_name)
                        .output();
                    copy_template(&project_name).expect("Template Python not successfully copied");
                }
                Err(_) => {
                    loop {
                        // begin user prompt
                        println!(
                            "Poetry installation not found. Would you like to install it? (y/n)"
                        );

                        // Print the prompt
                        print!(">> ");
                        io::stdout().flush().unwrap();

                        // Read a line of input
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();

                        // Remove the newline character
                        let input = input.trim();

                        // Handle the input
                        if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                            let _ = Command::new("pip").arg("install").arg("poetry").output();

                            let _ = Command::new("poetry")
                                .arg("new")
                                .arg(&project_name)
                                .output();
                            copy_template(&project_name)
                                .expect("Template Python not successfully copied");
                        } else {
                            panic!("Python environment cannot be set up without a Poetry installation.")
                        }
                    }
                }
            }
        }
        _ => panic!("Other languages are not yet supported."),
    }
}
