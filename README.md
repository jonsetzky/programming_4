# Course project for Programming 4

Front end for a chat server. Built with [dioxus](https://dioxuslabs.com/)
which uses WebView2 for rendering.

## Prerequisites

- Maven and JDK 18 or later
- Rust
  - Binaries can be found [here](https://rust-lang.org/learn/get-started/)
- (optional) `cargo-binstall`
  - `cargo install cargo-binstall`
- (optional) dioxus
  - `cargo install dioxus-cli` (or `cargo binstall dioxus-cli`)
  - Installation guide [here](https://dioxuslabs.com/learn/0.7/getting_started/)
    for possible problems

## Running the application

Start the [chat server](https://github.com/anttijuu/O4-server) by running:

```bash
cd server
mvn package
java -jar target/ChatServer-0.0.1-SNAPSHOT-jar-with-dependencies.jar chatserver.properties
```

The application can be started by running the following
command in the project's directory:

```bash
cargo run
```

## Commands for common tasks

| Task                    | Command        |
| ----------------------- | -------------- |
| Running dev environment | `dx serve`     |
| Running linter          | `cargo clippy` |
