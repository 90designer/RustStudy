### Rust安装慢（windows）
> 1. 在powerShell中分别执行一下命令：
> $ENV:RUSTUP_DIST_SERVER='https://mirrors.ustc.edu.cn/rust-static'
> $ENV:RUSTUP_UPDATE_ROOT='https://mirrors.ustc.edu.cn/rust-static/rustup'
> 2. 继续在此命令窗口下执行： rustup-init.exe

### Rust环境安装（Error: linker `link.exe` not found）
> 缺少C++环境
> https://blog.csdn.net/Alexhcf/article/details/107755195

### rustc  xxxx.rs 编译 (适合小项目编译)
> 生成两个文件

### Cargo 创建项目
> cargo new [projectName]

### cargo build 编译（开发时）
    cargo build --release 生产环境推荐
### cargo run  编译并执行项目
### cargo check 检查代码