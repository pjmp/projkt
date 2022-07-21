<div align="center">

# `projkt`

**Generate project related stuffs such as .gitignore, license, readmes and more**

[![crates.io](https://img.shields.io/crates/v/prokjt.svg?style=flat-square)](https://crates.io/crates/prokjt)
[![github.com](https://img.shields.io/github/license/pjmp/projkt?style=flat-square)](https://github.com/pjmp/projkt)
[![github.com](https://img.shields.io/badge/contributions-welcome-brightgreen?style=flat-square)](https://github.com/pjmp/projkt)
<!-- Get more badges at https://shields.io -->
</div>

## Usage

```bash

projkt <generator> # opens a fuzzy file picker

projkt <options> <generator> <args> # write output to file

# generate `.gitignore` file(s)
projkt gitignore

projkt --overwrite gitignore rust

# generate license file(s)
projkt --dest=~/Codes/repo license rust

```


## Installation


## Available generators

- gitignore
- license
- readme
