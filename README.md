# Axiom 0.1.0-dev ⚠️⚠️⚠️UNDER DEV⚠️⚠️⚠️

A **statically-typed, taint-aware scripting language** focused on secure, high-efficiency cybersecurity tasks.

## Basic Features (MVP)

* **Taint Tracking**: All external inputs are marked `Tainted<T>` by default.
* **Built-in Primitives**:

  * `read_stdin()` → reads user input as tainted data
  * `say(msg)` → print a string to stdout
  * `sanitize_shell_input(input)` → turns tainted data into clean data
  * `execute_command(cmd)` → runs a shell command only if input is clean
* **CLI Commands**:

  * `axiom run <file>`: execute a `.axo` script
  * `axiom check <file>`: static‐check scripts for taint or parse errors
  * Support for stdin: pass `-` as filename to read from piped input

## Installation

1. **Prerequisite**: [Rust & Cargo](https://rust-lang.org)
2. **Install Axiom**:

   ```bash
   git clone https://github.com/Ghostcar5153/axiom.git
   cd axiom/cli
   cargo install --path .
   ```
3. Ensure Cargo bin is on your `PATH` (e.g. `%USERPROFILE%\.cargo\bin` on Windows).

## Writing a Script

1. Create `hello.axo`:

   ```axo
   #!/usr/bin/env axiom run
   say("Hello, World!");
   ```
2. Run it:

   ```bash
   axiom run hello.axo
   ```
3. Or check without running:

   ```bash
   axiom check hello.axo
   ```

## Examples Directory

* **`examples/taint_pass.axo`**: trivial script that passes taint checks (`# just a comment`)
* **`examples/taint_fail.axo`**: demonstrates a taint violation (`execute_command(read_stdin());`)

## Development

Upcoming features under development:

* Expanded standard library (`sanitize_url`, `sanitize_path`, `getenv`, etc.)
* New DSL primitives (`ask`, `read_file`, `write_file`)
* CLI enhancements: `axiom fmt`, `axiom trace`
* Community examples and showcase tools

## License

This project is released under the **MIT License**.
