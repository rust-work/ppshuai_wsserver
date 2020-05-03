
1.配置代理
[source.crates-io]
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
[http]
check-revoke = false
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

2.Windows/Linux下安装openssl
Windows:
https://slproweb.com/products/Win32OpenSSL.html
下载安装，设置环境变量：OPENSSL_DIR=C:\OpenSSL-Win64
Linux:
sudo apt-get install openssl
sudo yum install openssl

3.运行init-windows.bat/init_linux.sh