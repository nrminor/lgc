# LGC: Let's Get Coding! 
[![Rust Build](https://github.com/nrminor/lgc/actions/workflows/build-rust.yaml/badge.svg)](https://github.com/nrminor/lgc/actions/workflows/build-rust.yaml) [![Open Source Files](https://github.com/nrminor/lgc/actions/workflows/open-source-starter.yml/badge.svg)](https://github.com/nrminor/lgc/actions/workflows/open-source-starter.yml)

A Rust-based command line tool that creates template files and configures a development environment for projects in a variety of languages.

This project is in its very early stages, but the vision is to be able to run a command like:
```
lgc init python3.11 --name my_project
```

The command will then create a project root directory, initialize git, initialize [Poetry](https://python-poetry.org/) for dependency management, and create [PEP8](https://peps.python.org/pep-0008/)-conforming a [Python script template](https://github.com/nrminor/lgc/blob/main/templates/python/template.py). This project is very much inspired by Rust's `cargo` package manager, which imposes opinionated formatting standards and idioms. This imposition can be tedious to comply with at first, but in the long run, it makes collaboration on Rust projects much easier. 

This project aims to do the same thing for a variety of languages that are common (or likely to become common) in scientific computing, including Python, R, Julia, Mojo, Go, Rust, Nextflow, Bash, and _maybe_ Perl. Users are encouraged to suggest additional languages, if desired, as an [Issue](https://github.com/nrminor/lgc/issues). Once discussed there, we welcome PRs from users who are more experienced with the desired language than us (which is virtually guaranteed!). The key to any successful PR is not just a template in the new language; the PR will need to successfully integrate the *tooling* that makes developing in that language organized, readable, performant, and [scientifically reproducible](https://osf.io/4pd9n/).

### A Disclaimer
Inevitably, the standards here will be strongly influenced by the contributors' preferences with respect to code styling and idioms (e.g., [writing Python like it's Rust](https://kobzol.github.io/rust/python/2023/05/20/writing-python-like-its-rust.html)). This is as it should be: having standards at all is more important than the specific nature of those standards. Still, any users who'd like to make suggestions about improving standards, environment setup tools, etc. should feel free to raise issues--or just fork the repo and make LGC your own!

That said, a goal of this project is to cite our sources, like any good scientist. Where possible, our templates will contain links to where a certain standard comes from, which will include the likes of:
 - [Python Enhancement Proposals (PEPs)](https://peps.python.org/pep-0000/)
 - Hadley Wickham's [*Advanced R*](https://adv-r.hadley.nz/index.html)
 - The official Julia [Performance Tips](https://docs.julialang.org/en/v1/manual/performance-tips/)
 
A markup file of of these sources may also be included for each language if that is more useful.

It should also be noted that this project is *not* a linter. However, projects initialized with LGC will come with a directory of suggested VSCode extensions. Included in the suggestions will be the relevant linters, which VSCode users are strongly encouraged to take advantage of.
