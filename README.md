# Course project for Programming 4

Front end for a chat server. Built with [dioxus](https://dioxuslabs.com/)
which uses WebView2 for rendering.

The app is built for desktop and it being currently implemented closely with `tokio`
makes it unable to be run directly on the web.

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

If you do not have the [chat server](https://github.com/anttijuu/O4-server) running
on port `10000` download it by fetching the submodule with:

```bash
git submodule update
```

## Running the application

If you do not have the [chat server](https://github.com/anttijuu/O4-server) running
on port `10000`, start it by running:

```bash
cd server
mvn package
java -jar target/ChatServer-0.0.1-SNAPSHOT-jar-with-dependencies.jar chatserver.properties
```

The application can be started by running the following
command in the project's directory:

```bash
dx serve
```

> Note that `cargo run` will not work as the application doesn't have permissions to load assets from the assets folder.

## Commands for common tasks

| Task                    | Command               |
| ----------------------- | --------------------- |
| Running dev environment | `dx serve`            |
| Running linter          | `cargo clippy`        |
| Bundling the project    | `dx bundle --desktop` |

## Misc Notes

- Multiple messages can be sent in a single message separated by newline `\n`?

## TODO

- Add dividers for message history view that display:
  - When date changes
  - When topic changes
