alias b := build
alias r := run
alias t := test
alias v := version

default:
    just --list

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

# Set or get version
[doc("Set or get \u{001b}[4mv\u{001b}[24mersion")]
version VERSION="none":
    @if [ "{{VERSION}}" = "none" ]; then \
        git describe --tags --abbrev=0; \
    else \
        just set-version {{VERSION}}; \
    fi

# Set version
[doc("Set version")]
set-version VERSION:
    # Set version in Cargo.toml
    sed -i "s/^version = \".*\"/version = \"{{VERSION}}\"/" Cargo.toml
    cargo update
    # Add changes to git
    git add Cargo.toml Cargo.lock
    # Commit changes
    git commit -S -m "Bump version to v{{VERSION}}"
    # Tag version
    git tag -s v{{VERSION}} -m "Version v{{VERSION}}"
