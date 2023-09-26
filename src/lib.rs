pub mod lib {

    // Expose the core LGC commands
    pub mod lgc_commands;

    // Available languages
    pub mod golang;
    pub mod julia;
    pub mod rlang;
    pub mod python;
    pub mod mojo;
    pub mod rust;
    pub mod nextflow;

    // pulling in tooling calls
    pub mod tooling_calls {

        pub mod call_poetry;
        pub mod call_pkg;
        pub mod call_cargo;
        pub mod call_rstudio;

    }

}
