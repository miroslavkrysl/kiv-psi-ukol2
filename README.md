# KIV/PSI - úkol 2

Author: Miroslav Krýsl

## Description

Simple HTTP server. It receives only GET requests and generates responses.
It supports only few URIs. All supported URIs are enlisted on the root page
of the server (`GET /`)

The server listening address defaults to `0.0.0.0:8080`. You can change it in the `main.rs` file, but then you must rebuild the project. 

## Run

Binaries for Linux and Windows are included in `bin` directory.

## Install, build, run

It is written in Rust. Rust projects are managed with `cargo` command which needs
the `Cargo.toml` file in project root.

You can install Rust toolchain with your preferred way, but the simplest way
is to install Rust via `rustup` (https://rustup.rs/). You should follow instructions
on the website and then instructions in the installation script.

If you have Rust installed, just run in project root:

`cargo run` for building and running.


Or only for building:

`cargo build`
