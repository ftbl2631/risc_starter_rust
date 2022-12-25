#riscv-rust
使用GD32VF103开发板进行嵌入式rust的开发

##第一章 toolchain建立

### 1.1 工具
程序开发需要：
1）选择开发语言（这里也就是Rust语言）
2）开发的环境 （在linux系统下选择vscode作为代码编辑器）
3）编译、链接、生成
Rust下的开发依赖于使编译器rustc，但使用rustup作为toolchain管理工具，使用cargo作为包管理工具。
rustup是Rust的安装程序，也是版本管理的工具。
安装rustup之后，默认安装rustc编译器
4）调试、下载、应用

### 1.2 工具安装
rustup安装（默认安装Rust最新稳定版本）
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
安装后检查
rustup --version
执行之后可以看到已经安装的rustup和rustc的版本号，再用cargo --version检查，看到cargo也已经安装。
卸载
rustup self uninstall

rustc介绍
