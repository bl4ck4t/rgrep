# rgrep
A simple Rust CLI tool for searching a text pattern within a file, designed for efficiency and speed.

## Description
rgrep allows users to search for specific patterns in text files.

## Build
To compile the project and generate the executable,

run:
```sh
cargo build
```

## Usage
The basic syntax for running the tool is:
```sh
rgrep <options> <pattern> <file>
```
options :
```sh
--ignore-case OR -i: Performs a case-insensitive search.
```

Examples: 

## Basic search
```sh 
rgrep hello test.txt
```

## Case-insensitive search
```sh
rgrep --ignore-case hello test.txt
```

## Behavior
- Memory Efficient: Reads the file line by line using buffered I/O; it does not load the entire file into memory.

- Output: Prints every line that contains the specified pattern to the standard output.

- Safety: Written entirely in safe Rust—no unsafe code used.

## Tests
To run the internal unit tests and integration tests:
```sh
cargo test
```

## Project Structure
The project follows a modular library-first approach:

```sh
src/
  ├── main.rs    # CLI entry point (handles I/O & errors)
  ├── lib.rs     # Library interface
  ├── config.rs  # Argument parsing (using clap)
  └── search.rs  # Core search algorithms

tests/
  └── search_tests.rs  # Integration tests for search logic
```
Note: The core logic is decoupled from the CLI interface and implemented as a library, allowing for easy testing and potential reuse.
