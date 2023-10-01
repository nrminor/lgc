use crate::lib::copy_template;
use std::io::{self, Write};
use std::process::Command;

pub fn build_env(project_name: &String) -> std::io::Result<()> {
    let check_poetry = Command::new("poetry").arg("--version").output();

    match check_poetry {
        Ok(_) => {
            let _ = Command::new("poetry")
                .arg("new")
                .arg(&project_name)
                .output();
            copy_template(&project_name, "Python")?;
        }
        Err(_) => {
            loop {
                // begin user prompt
                println!("Poetry installation not found. Would you like to install it? (y/n)");

                // Print the prompt
                print!(">> ");
                io::stdout().flush()?;

                // Read a line of input
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                // Remove the newline character
                let input = input.trim();

                // Handle the input
                if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
                    let _ = Command::new("pip").arg("install").arg("poetry").output();

                    let _ = Command::new("poetry")
                        .arg("new")
                        .arg(&project_name)
                        .output();
                    copy_template(&project_name, "Python")?;
                } else {
                    panic!("Python environment cannot be set up without a Poetry installation.")
                }
            }
        }
    }
    Ok(())
}
