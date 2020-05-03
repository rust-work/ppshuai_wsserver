copy windows\sqlite3.lib %USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ /Y
cargo install diesel_cli --no-default-features --features sqlite --force

echo "DATABASE_URL=test.db" > .env
diesel migration run