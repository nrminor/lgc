use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

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
    let src_file = root_dir.join("templates/Go/template.go");

    // Get the current working directory
    let dest_dir = env::current_dir()?;

    // Form the path to the destination file
    let dest_file = dest_dir.join(format!("{}/{}/template.go", project_name, project_name));

    // Copy the file
    fs::copy(src_file, dest_file)?;

    Ok(())
}

pub fn build_env(project_name: &String) -> std::io::Result<()> {
    let check_go = Command::new("go").arg("--help").output();

    match check_go {
        /*
        PLEASE IGNORE THE FOLLOWING FOR NOW.
        */
        Ok(_) => {
            let _ = Command::new("poetry")
                .arg("new")
                .arg(&project_name)
                .output();
            copy_template(&project_name)?;
        }
        Err(_) => {
            loop {
                // begin user prompt
                println!("Go installation not found. Would you like to install it? (y/n)");

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
                    copy_template(&project_name)?;
                } else {
                    panic!("Python environment cannot be set up without a Poetry installation.")
                }
            }
        }
    }
    Ok(())
}
