# deciduously-com

Rewrite of my personal (unfinished) website at [deciduously.com](http://deciduously.com), this time in `actix_web`.

## Usage

Go to the website

## Develop

Requires stable Rust and Docker.  Use `cargo run -- publish` to write any markdown files in `/drafts` to Tera (Jinja) templates,. and `cargo run -- serve` to serve the site.  Run `docker build -t deciduously-com` to build the docker image.

Takes either `BUILD=dev` or `BUILD=prod`, defaulting to `dev`.

## Externs

* Dots - run the `wasm-pack init`, `cd www && yarn webpack`, copy everything in `www/dist` to `static/externs/dots`.