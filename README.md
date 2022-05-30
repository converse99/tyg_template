# A rapid start-up template for Rust command line projects

When starting a Rust command line project there are certain key features that
are often required in all such projects:

- The ability to parse the command line
- Sensible and informative error trapping

The `tyg_template` project aims to provide both of these features enabling a consistent and
simple start to any command line coding project. Rather than creating a new command line parser
I've elected to use the excellent ['clap'](https://docs.rs/clap/latest/clap) crate.

In case you're wondering, 'tyg' stands for 'there you go'.

## Examples

*In the following examples, a Unix like operating system is assumed.*

The best way to use this is simply to do a git clone and then rename everything to suit your
own coding project. I would not recommend using it in its raw form directly, however, before
modifying the template to suit your own application, you could try running the sample
application by entering the following at the command prompt:

```text
$ cargo run -- --help
```

This will display the help option.

You could also try the following:

```text
$ cargo run -- fail
tyg_template: src/lib.rs:122:9: Error thrown to demonstrate the error handling process
```

This is what I call a disclosed error showing the name of the source file and where in the
source file the error occured.

It is also possible to throw non-disclosed errors in which the origin is not disclosed to the
end user, however, this can be overridden by compiling with the `disclose` feature enabled.
Non-disclosed errors are useful to provide feedback to the end user in which the source
location is not disclosed to the user.

```text
$ cargo run -- fail --bare
tyg_template: Error thrown to demonstrate the error handling process
```

The same again but this time compiled with the `disclose` feature enabled.

```text
$ cargo run --features=disclose -- fail --bare
tyg_template: src/lib.rs:132:9: Error thrown to demonstrate the error handling process
```

Notice that the error message now shows the location of the error.

In general, disclosed errors are ideal for debugging purposes, so during a debug session I
would recommend compiling the application with the `disclose` feature enabled.

## Usage

As mentioned previously, the best way to use this is simply to do a git clone and then rename
everything to suit your own coding project. My recommendation is that the cli and run functions
in lib.rs are moved to the main program (main.rs). This enables you to start with a clean new
lib.rs. Using this configuration confines the command line interface and top level error
trapping to main.rs. The library crate will then be responsible for the overall functionality
of the application.

## Initial File Structure

```text
 tyg_template --|
                |- Cargo.toml
                |- Cargo.lock
                |- LICENCE
                |- README.md
                |- src
                    |- main.rs {The main program}
                    |- lib.rs {The library crate root}
                    |- error.rs {The error handler}
```

