# About

This project is a [Cambridge Style Pseudocode](https://www.cambridgeinternational.org/Images/697401-2026-syllabus-legacy-notice.pdf) interpreter. It is meant for educational purposes only.

Cambridge mandates writing algorithms and programs in pseudocode, on paper for students appearing for [AS Level Computer Science (9618)](https://www.cambridgeinternational.org/programmes-and-qualifications/cambridge-international-as-and-a-level-computer-science-9618/) and [IGCSE Level Computer Science (0478)](https://www.cambridgeinternational.org/programmes-and-qualifications/cambridge-igcse-computer-science-0478/) exams.

But doesn't an interpreter defeat the whole purpose of pseudocode? I believe it does not, an interpreter would help students to check their dry-runs as well as help with learning the syntax through meaningful error messages.

# Usage Guide

## Install

1. Install using Cargo

```sh
cargo install --git https://github.com/nouritsu/rs-pseudocode
```

2. Run and pass file to be interpreted as first argument

```sh
rs-pseudocode <file>
```

## Uninstall

Uninstall using Cargo

```sh
cargo uninstall rs-pseudocode
```

## Build from Source

1. Clone the repository

```sh
git clone https://github.com/nouritsu/rs-pseudocode
```

2. Build using Cargo

```sh
cargo build --release
```

Executable located in target/release

3. Run and pass file to be interpreted as the first argument

```sh
./<path-to-executable> <file>
```

To build and interpret a file use -

```sh
cargo run --release -- <file>
```
