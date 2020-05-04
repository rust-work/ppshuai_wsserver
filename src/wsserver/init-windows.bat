#copy 安装目录\libmysqlclient.lib %USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ /Y
copy windows\sqlite3.lib %USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\ /Y
cargo install diesel_cli --no-default-features --features sqlite --features mysql --force

#echo "DATABASE_URL=mysql://user:password@127.0.0.1/dbname" > .env
echo "DATABASE_URL=test.db" > .env

diesel migration run
