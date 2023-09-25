# LGC: Let's Get Coding! 
[![Rust Build](https://github.com/nrminor/lgc/actions/workflows/build-rust.yaml/badge.svg)](https://github.com/nrminor/lgc/actions/workflows/build-rust.yaml) [![Open Source Files](https://github.com/nrminor/lgc/actions/workflows/open-source-starter.yml/badge.svg)](https://github.com/nrminor/lgc/actions/workflows/open-source-starter.yml)

A command line tool that creates template files and configures a development environment for projects in a variety of languages

This project is in its very early stages, but the vision is to be able to run a command like:
```
lgc init python3.11 --name my_project
```

The command will then create a project root directory, initialize git, initialize Poetry for dependency management, and create [PEP8](https://peps.python.org/pep-0008/)-conforming a [Python script template](https://github.com/nrminor/lgc/blob/main/templates/template.py). This project is very much inspired by Rust's `cargo` package manager, which imposes opinionated formatting standards and idioms. This imposition can be tedious to comply with at first, but in the long run, it makes collaboration on Rust projects much easier. This project aims to do the same thing for a variety of languages that are common (or may become common) in scientific computing, including Python, R, Julia, Mojo, Go, Rust, and Perl.

Inevitably, the standards here will be strongly influenced by the contributors' preferences with respect to code styling and idioms (e.g., [writing Python like it's Rust](https://kobzol.github.io/rust/python/2023/05/20/writing-python-like-its-rust.html)). This is as it should be: having standards at all is more important than the specific nature of those standards. Still, any users who'd like to make suggestions about improving standards, environment setup tools, etc. should feel free to raise issues--or just fork the repo and make LGC your own!
