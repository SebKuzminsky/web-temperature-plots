A small project to play with web technologies in Rust.

This project builds a small wasm web app using [yew](https://yew.rs/)
and [plotters](https://plotters-rs.github.io/home/#!/).  The web app
runs in a browser and gets temperature information over websockets from
a small server running on the network.


# Build

Build the host-side programs (server and client) in the host-side
directory with `cargo build`.  Start the server.

Build the web app, either with `trunk serve` or with `trunk build
--public-url http://example.com/path/to/app`.  The serve command will
start a temporary web server

Then point a web browser at the trunk url.
