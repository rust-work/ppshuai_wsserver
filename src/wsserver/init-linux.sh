#!/bin/sh
#cp linux/libsqlite3.a $HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/ -f
cargo install diesel_cli --no-default-features --features sqlite --force
echo "DATABASE_URL=test.db" > .env
diesel migration run