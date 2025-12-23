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
cargo run
```

## Commands for common tasks

| Task                    | Command                                                    |
| ----------------------- | ---------------------------------------------------------- |
| Running dev environment | `dx serve`                                                 |
| Running linter          | `cargo clippy`                                             |
| Updating database       | https://diesel.rs/guides/getting-started                   |
| Running db migration    | `diesel migration generate --diff-schema <migration-name>` |

`

# Peer-to-peer architecture

When a client joins it sends Hello packet publicly. All connected clients will respond to that hello packet using direct messages.

All connected clients are expected to have a consensus of existing channels. The joining client asks one of the connected clients for channels using RequestChannels packet. And then the responding client sends RespondChannels packet to the public channel, which all clients will handle. This results in all clients having matching channel lists.

## Syncing channels between clients

A client can send RequestChannels packet, which contains uuids of the channels they already know. All other clients respond to that packet with channels that aren't in that request.

The RequestChannels packet is sent to all new clients once they join the server.

TODO (This isn't implemented): A channel name collision can happen. This is resolved by dropping the channel with latter creation timestamp.
