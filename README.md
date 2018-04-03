# busted.rs [![Build Status][travis.svg]][travis]

A collection of utilities written in Rust.

The goal of this project is to demonstrate solutions for various core features one would be required to build for a
full-fledged application. Here is a list of ideas that may or may not make it into this project:

 - Web Server: Serve a local directory over HTTP.
 - Web Client: A simplified `curl` utility
 - Web Crawler: Crawl a web page and extract useful information out of it.
 - Regex Find: A `find` utility which uses regular expressions to match files and directories recursively.

All of these solutions should use standard practices such as logging and command line argument parsing. Additional
features may be added at any time, I will try to keep this list up to date.

## Building and Running

We ship a Vagrant VM which is provisioned with everything needed for Rust development. To use this environment, simply
`vagrant up` and `vagrant ssh` into the machine. The VM will be provisioned with `rustup`, `cargo`, `make`, `gcc`, etc.

Alternatively, if you have Cargo installed outside of a VM and are fine without the isolation that a VM provides, this
project can be easily built using `cargo build` and `cargo run`, just as you'd expect.

Travis CI support is also included for running tests and ensuring builds succeed.

## License

Licensed at your discretion under either:

 - [MIT License](./LICENSE-MIT)
 - [Apache License, Version 2.0](./LICENSE-APACHE)

 [travis]: https://travis-ci.org/naftulikay/busted.rs
 [travis.svg]: https://travis-ci.org/naftulikay/busted.rs.svg?branch=master
