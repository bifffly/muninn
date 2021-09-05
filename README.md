# rydja-server

This is a basic implementation of a ryðja server in Rust. It is based loosely off of the HTTP server tutorial in the Rust documentation.

## Serving files

Currently, all files to be served are stored in `/example` and are served on `localhost`, or `127.0.0.1`. Updates on config files and non-local serving are in the works.

To serve your files, run `cargo run` in the root directory.
