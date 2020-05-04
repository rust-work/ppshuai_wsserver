#!/bin/sh
#cp 安装目录/libsqlite3* $HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/ -f
cp linux/libsqlite3.a $HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/ -f
cargo install diesel_cli --no-default-features --features sqlite --features mysql --force
#echo "DATABASE_URL=mysql://root:123456@10.0.2.252/ppsdb" > .env
echo "DATABASE_URL=test.db" > .env
diesel migration run
