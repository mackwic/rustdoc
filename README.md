# rustdoc

A tool for documenting Rust.

> *NOTE*: This is not the "real" `rustdoc`. This is a prototype of a possible
> replacement. The `rustdoc` you get with Rust lives in
> [the Rust repo](https://github.com/rust-lang/rust), in the `src\librustdoc`
> directory.

Specifically, you can run `rustdoc` inside the root of a crate, and it will
produce HTML, CSS, and Javascript into the `target\doc` directory. Open
`target\doc\index.html` to check it out.

## Project structure

There are three top-level directories that are important: `src` contains the main source
code for `rustdoc`. However, it will grab HTML, CSS, and JS from the `frontend` directory,
which is where all of that stuff is developed. Finally, the `example` directory contains
a sample crate that you can use to try out `rustdoc`; we add stuff to it as we add support
for various things into `rustdoc` itself.

### The backend

The backend, located in `src`, is written in Rust. `rustdoc` is effectively a compiler,
but instead of compiling source code to machine code, it compiles source code to JSON.
Here's how it does it:

1. It shells out to `cargo` to generate "save analysis files", which are placed in
   `target\rls`.
2. It reads those save analysis files with the `rls-analysis` crate. As you may be able
   to guess from the name, this is pretty much why it exists!
3. It goes through the processed information and generates a sort of "docs AST"; starting
   at the root of your crate and branching from there.
4. It converts this "docs AST" to JSON, more specifically, [JSON API](https://jsonapi.org).
5. It writes out this JSON to the `target\doc` directory of the crate that
   it's documenting.
6. It writes out some HTML/CSS/JS from the frontend `target\doc` too.

Well, this is [how it's going to
work](https://github.com/steveklabnik/rustdoc/issues/11), anyway: the code
isn't exactly super clean at the moment. More work to do!

### The frontend

The frontend is currently implemented with [Ember](https://emberjs.com/). Its source
code is in the `frontend` directory.

The first thing that the frontend does is in `frontend\app\routes\application.js`. This
route runs before anything else, and it makes a request to grab a `data.json` file, which
is generated by the back end. This loads up all of the docs into `ember-data`, which
drives the rest of the site.

One other slightly unusual aspect of the frontend: normally, you'd have the `dist`
directory ignored, as you don't want to commit generated files. In this case, though,
we don't want `ember` to be a dependency of installing `rustdoc`, and so we do commit
those generated files.

## Usage

Currently, it only builds the given example. Do it as follow:

```
cargo run --release -- --manifest-path=example
```

Then open a web browser and open "rustdoc/example/target/doc/index.html".

## Known issues (and their solution)

 * "javascript error: data.json isn't found": go to `example/target/doc` and then run `python -m SimpleHTTPServer`. Then go to the given URL above.

## Contributing

We'd love your help with making `rustdoc` better! It's currently very early days, so
there's a lot to do. Here's a quick overview:

1. `rustdoc` is dual licensed under the MIT and Apache 2.0 licenses, and so contributions
   are also licensed under both.
2. Contributions go through pull requests to the `master` branch.
3. Check out the [issue tracker](https://github.com/steveklabnik/rustdoc) to follow the
   development of `rustdoc`.

For more details, see the `CONTRIBUTING.md` file.