alias b := build
alias r := run
alias t := test

# Build release binary
[doc("\u{001b}[4mB\u{001b}[24muild release binary")]
build:
    cargo build --release --features="cli"

# Compile and run
[doc("Compile and \u{001b}[4mr\u{001b}[24mun")]
run *args:
    cargo run --features="cli" -- {{args}}

# Tests
[doc("\u{001b}[4mT\u{001b}[24mests")]
test *args:
    cargo test {{args}}
