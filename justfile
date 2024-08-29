# Watch for changes and reload the server.
watch:
    cargo watch -x 'run --release'

# Run the server
run:
    cargo run --release

# Install some useful tools, you'll need these
# to run `just watch`
install-tools:
    cargo install cargo-binstall
    cargo binstall cargo-watch
