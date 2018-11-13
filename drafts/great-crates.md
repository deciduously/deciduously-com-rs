# Crates I Have Known And Loved

There are many reasons I enjoy writing Rust but close to the top is how well-designed the tooling is.  As far as I'm concerned `cargo` is everything I wanted out of a package manager and then some, and you can create your own subcommands if you need.

[The Book](https://doc.rust-lang.org/stable/book/2018-edition/index.html) does a fantastic job of getting you up and running, but it doesn't touch the crate ecosystem.  Now that I'm several projects deep into my Rust journey I've settled on a few "must-haves" that my blank projects almost always end up with.  I wish I'd had this list from the get-go - some of this functionality I had been hand-implementing for far too long before I found the solution.  If you've got some more, let me know!

## `error-chain`

[docs.rs](https://docs.rs/error-chain/0.12.0/error_chain/)

This is pretty much always my first addition to any project.

One of my favorite features of Rust is the `?` operator.  If an operation returns a `Result<T, E>` you can just tack a question mark on it and get the logic you usually want - a success will continue execution and a failure will early-return an `Err`.

However, this only works if the function you're in returns the same exact type.  This often isn't the case - you may have an app-specific error type for the containing function but inside call something from `std::io` - in this case it won't work unless you're implementing error-type conversions all over the place yourself.  The `error-chain` crate lets you, well, chain errors:

```rust
fn get_dir_listing(dir_str: &str) -> errors::Result<Vec<PathBuf>> {
    let dir_listing: Vec<PathBuf> = read_dir(dir_str)
                                    .chain_err(|| "could not read dir!")?
    //etc...
}
```

Now my error is just a simple string, and I'll see it in the stacktrace connected to the underlying `IO::Error` that was generated!  It also provides a custom `Result<T>` type that automatically uses your error chain.  Which is nice.

In application code using strings like that generally gets the job done but for a library you'll want a more robust custom error type - this crate also provides an opinionated structure for defining one should you so choose.  I've so far been content to leave the setup completely empty - all you need is the below in your main.rs and you're good to go:

```rust
extern crate error_chain;

mod errors {
    error_chain!{}
}
```

Then you change `main()` - this is straight from the docs:

```rust
fn main() {
    if let Err(ref e) = run() {
        error!("error: {}", e);

        for e in e.iter().skip(1) {
            debug!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            trace!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
```

In the above snippet, `run()` is really our main function but it's properly error-chained (returns an `errors::Result`)- your whole app is covered this way.  The `if let` syntax expresses exactly the behavior we want in a concise, clear manner.

Just add `use errors::*` anywhere you need.

## `structopt`

[docs.rs](https://docs.rs/structopt/0.2.12/structopt/)

Structopt feels like cheating.  The gold standard for scaffolding command-line apps is [clap](https://clap.rs/). Structopt makes it *even easier* than `clap` already does.  It lets you write the following (from the doc link):

```rust
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    speed: f64,
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}
```

It will automatically generate a `clap::App` for you! The triple-slashed docstrings in the snippet will become the help line for each argument.  To compare, the below is the "usual" method from a project I built before I was Enlightened:

```rust
let matches = App::new("ar-bot")
        .version(VERSION)
        .author("deciduously <ben@deciduously.com>")
        .about("Batching of auto email alerts")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .takes_value(true)
                .help("Specify an alternate toml config file"),
        )
        .arg(
            Arg::with_name("digest")
                .short("d")
                .long("digest")
                .takes_value(false)
                .help("Finalizes a digest with the emails in the brain. Make sure to preview first!")
        )
        .arg(
            Arg::with_name("email")
                .short("e")
                .long("email")
                .takes_value(false)
                .help("Placeholder command for developing email functionality"),
        )
        .arg(
            Arg::with_name("preview")
                .short("p")
                .long("preview")
                .takes_value(false)
                .help("Displays the current contents of the batch"),
        )
        .arg(
            Arg::with_name("report")
                .short("r")
                .long("report")
                .takes_value(false)
                .help("Daily report comparing inputs to outputs for the day"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Set RUST_LOG verbosity.  There are three levels: info, debug, and trace.  Repeat the flag to set level: -v, -vv, -vvv.")
        )
        .get_matches();
```

It's a lot more typing for the same endgame, and at the end everything is already handily stored in your `Opt` struct.  Struct-opt :)


## `envy`

[github](https://github.com/softprops/envy)

This crate is similar to `structopt` but for environment variables.  You define a struct and it can auto-fill it with any environment variables present:

```rust
#[macro_use]
extern crate serde_derive;
extern crate envy;

#[derive(Deserialize, Debug)]
struct Config {
  foo: u16,
  bar: bool,
  baz: String,
  boom: Option<u64>
}

fn main() {
    match envy::from_env::<Config>() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error)
    }
}
```

Now it will automatically read the FOO, BAR, BAZ, and BOOM env vars at runtime.

It's another task that's not necessarily difficult to do by hand but it's tedious and you're likely doing it a lot, over and over again.

## `serde`


[serde.rs](https://serde.rs)

Serde at least to me feels so intertwined with Rust I'm sure this isn't a surprise to anyone, but it's a seriously solid solution.  Super sound, stupendously speedy.  Say that five times fast.

Mouthful aside, serde is a no-brainer when you need to do any serializing or deserializing, which is...usually.  I'm not even including a snippet because in most cases it can derive all the functionality you need with a single annotation, and it's not hard to hand-implement the traits yourself if you need.  It's fast and simple!

## `cargo-watch`

[github](https://github.com/passcod/cargo-watch)

Watch your files for changes and re-run the `cargo` subcommands of your choosing with, for example, `cargo watch -x test -x run`.  I don't have anything more to say, that pretty much speaks for itself.  This is a must-have for me.

## `pretty_env_logger`

[github](https://github.com/seanmonstar/pretty-env-logger)

This is kind of a twofer - it's a colorful wrapper around [`env-logger`](https://docs.rs/env_logger/0.5.13/env_logger/).  I didn't start using the latter until I found this crate, though, and the colors are nice.

`env-logger` allows you to set the logging output level via an environment variable.  Then you use the macros from the [`log`](https://docs.rs/log/0.4.6/log/) crate: `info!`, `warn!`, `debug!`, `trace!`.  When you run your code, only those in the level specified will display.  This is a serious step up over println debugging - you can leave your debug printouts in and then just set a "verbose" flag to suppress them in normal usage.

I'm sure there's a better way to do this, but I've been dropping the below function into each project that uses the logging tools and it's working well enough for me:

```rust
fn init_logging(level: u64) -> Result<()> {
    let verbosity = match level {
        0 => "warn",
        1 => "info",
        2 => "debug",
        3 | _ => "trace",
    };
    if verbosity == "trace" {
        set_var("RUST_BACKTRACE", "1");
    };
    set_var("RUST_LOG", verbosity);
    info!(
        "Attempting to set logger to {}",
        var("RUST_LOG").chain_err(|| "Failed to set verbosity level")?
    );
    pretty_env_logger::init();
    info!(
        "Set verbosity to {}",
        var("RUST_LOG").chain_err(|| "Failed to set verbosity level")?
    );
    Ok(())
}
```

It simplifies the levels a little to make it easier to use with a verbosity flag that takes 0, 1, 2, or 3 levels (`-v`, `-vv`, `-vvv`), and makes sure if you've set `RUST_BACKTRACE` that you're getting the `trace` level no matter what, and will set `RUST_BACKTRACE` for you if you pass it `-vvv`.

## `pretty_assertions`

[github](https://github.com/colin-kiegel/rust-pretty-assertions)

This is in a similar vein as `pretty_env_logger`.  It's a drop-in replacement for `assert_eq!` with colored output.  You just add the crate, no code changes required at all.  Of course, you're a responsible developer and are using `assert_eq!` all over the place - this just makes the output a bit easier to read.

## `indicatif`

[link](https://github.com/mitsuhiko/indicatif)

This crate provides multiple progress bars and spinners to use in your command-line apps.  See the github README for some animated examples.

## `r2d2`

[docs.rs](https://docs.rs/r2d2/0.8.2/r2d2/)

This crate if likely familiar if you've done any database work, but I'll throw it in anyway because it's nice.  It's a connection pool for your database.  From the readme:

> Opening a new database connection every time one is needed is both inefficient and can lead to resource exhaustion under high traffic conditions. A connection pool maintains a set of open connections to a database, handing them out for repeated use.

It's backend-agnostic and easy to drop in to your app.  An adapter exists to use it easily with the [`diesel`](https://diesel.rs) ORM.  Now instead of connecting directly to your DB when you need it, you ask for a connection from the Pool instead and it all works as expected.  I love minimal-effort drop-in performance gains, don't you?

## `pest`

[pest.rs](https://pest.rs)

This won't be useful in all projects, but it's my current go-to for parsing needs.  It's much easier to use than a do-it-yourself parser-combinator library like `nom`.  You define your whole grammar in a separate file.  Then in your Rust code:

```rust
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;
```

As an example, here's a small (in progress) prefix calculator's grammar:

```
COMMENT = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
WHITESPACE = _{ " " }

num = @{ int ~ ("." ~ digit*)? }
    int = { ("+" | "-")? ~ digit+ }
    digit = { '0'..'9' }

symbol = @{ "+" | "-" | "*" | "/" | "%" | "^" | "add" | "sub" | "mul" | "div" | "rem" | "pow" | "max" | "min" | "list" | "eval" }

sexpr = { "(" ~ expr* ~ ")" }

qexpr = { "{" ~ expr* ~ "}" }

expr = { num | symbol | sexpr | qexpr }

blispr = { SOI ~ expr* ~ EOI }
```

And the corresponding code to read the parsed input:

```rust
fn lval_read(parsed: Pair<Rule>) -> Box<Lval> {
    match parsed.as_rule() {
        Rule::blispr | Rule::sexpr => {
            let mut ret = lval_sexpr();
            for child in parsed.into_inner() {
                // here is where you skip stuff
                if is_bracket_or_eoi(&child) {
                    continue;
                }
                ret = lval_add(&ret, lval_read(child));
            }
            ret
        }
        Rule::expr => lval_read(parsed.into_inner().next().unwrap()),
        Rule::qexpr => {
            let mut ret = lval_qexpr();
            for child in parsed.into_inner() {
                if is_bracket_or_eoi(&child) {
                    continue;
                }
                ret = lval_add(&ret, lval_read(child));
            }
            ret
        }
        Rule::num => lval_num(parsed.as_str().parse::<i64>().unwrap()),
        Rule::symbol => lval_sym(parsed.as_str()),
        _ => unreachable!(),
}
```

This library is incredibly easy to use.  I love how it maintains your grammar completely separate from your code, and the PEG format is easy to follow.  Give it a whirl!

## `actix_web`

[actix.rs](https://actix.rs)

This isn't so much for use in the general case, but if I'm writing a webserver this is what I reach for, without hesitation.  A lot of choice between webservers in the Rust ecosystem does boil down to personal taste, but I like how fast this one is and that it's been running on the stable branch since it launched.

I haven't had the opportunity to use the actor model without the webserver, but it looks great too!

## `ggez`

[ggez.rs](http://ggez.rs/)

This is a game framework inspired by [LÖVE](https://love2d.org/), with a Rustier API.  It's quite easy to get up and running with - perfect for prototyping.

For a larger game I'd recommend looking at [Amethyst](https://www.amethyst.rs/).  It seems to be the most promising engine at the moment and wraps [`specs`](https://github.com/slide-rs/specs), an Entity-Component System.  `specs` is the *only* ECS I've ever personally used so I can't really compare it to anything else..but that said, I think it's nice?

## svgbob

This one gets honorable mention because its just cool, not because its a library.  [Go check it out](https://ivanceras.github.io/svgbob-editor/) - it converts ASCII diagrams into SVG.