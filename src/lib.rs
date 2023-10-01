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
}
