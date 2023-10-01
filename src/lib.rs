pub mod lib {

    // Expose the core LGC commands
    pub mod lgc_commands;

    // Available languages
    pub mod golang;
    pub mod julia;
    pub mod mojo;
    pub mod nextflow;
    pub mod python;
    pub mod rlang;
    pub mod rust;

    // pulling in tooling calls
    pub mod tooling_calls {

        pub mod call_cargo;
        pub mod call_pkg;
        pub mod call_poetry;
        pub mod call_rstudio;
    }

    /*
    Enums, implementations, structures, and functions that are re-used
    across the whole package
    */
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

    use std::collections::HashMap;
    use std::env;
    use std::fs;

    fn copy_template(project_name: &String, language: &str) -> std::io::Result<()> {
        // Get the current executable's path
        let exe_path = env::current_exe()?;
        let root_dir = exe_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        let mut lang_abbrevs = HashMap::new();

        // Insert language-extension pairs into the HashMap
        lang_abbrevs.insert("Julia", "jl");
        lang_abbrevs.insert("R", "r");
        lang_abbrevs.insert("Python", "py");
        lang_abbrevs.insert("Go", "go");
        lang_abbrevs.insert("Rust", "rs");
        lang_abbrevs.insert("Perl", "pl");
        lang_abbrevs.insert("Mojo", "mojo");

        let lang_abbrev = lang_abbrevs
            .get(language)
            .expect("Unsupported language requested.");

        // Form the path to the source file relative to the executable
        let src_file = root_dir.join(format!("templates/{}/template.{}", language, lang_abbrev));

        // Get the current working directory
        let dest_dir = env::current_dir()?;

        // Form the path to the destination file
        let dest_file = dest_dir.join(format!(
            "{}/{}/template.{}",
            project_name, project_name, lang_abbrev
        ));

        // Copy the file
        fs::copy(src_file, dest_file)?;

        Ok(())
    }
}
