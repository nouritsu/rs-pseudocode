# About

This project is a Cambridge Style Pseudocode interpreter. It is meant for educational purposes only.

Cambridge mandates writing algorithms and programs in pseudocode, on paper for students appearing for AS Level Computer Science (9618) and IGCSE Level Computer Science (0478) exams.

But does this not defeat the whole purpose of pseudocode? I believe it does not, an interpreter would help students to cross check their dry-runs as well as help them get better at learning the syntax through meaningful error messages.

# Usage

# Running

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
./executable ./file.pseudo
```

To build and interpret a file use -

```sh
cargo run --release -- ./file.pseudo
```
