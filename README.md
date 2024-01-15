# Convert_Rust_code_To_Python_module
通过Rust代码快编译为pyd，替换Python系统模块或者自定义的模块，从而提高执行效率！

# 具体操作步骤

## Rust操作（如果已经安装直接忽略！！！）

### 1、安装Rust环境

[官网-安装教程](https://www.rust-lang.org/zh-CN/tools/install "官网-安装教程")

- Windows系统安装程序
[点击下载Rustup](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe "Windows系统的Rustup")
Windows系统的前置条件，需要安装【VS studio】，如果没有安装，直接选择【1】默认即可！

- Linux/MacOS系统安装命令
```cmd
# 国内建议先设置镜像然后在安装会快一些，执行下面两行命令！
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 执行安装命令（参考官网）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 2、配置Cartes镜像
```cmd
# 打开 ~/.cargo/config  没有就自行创建
vi[vim] ~/.cargo/config

# 添加内容如下，直接复制粘贴！
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```
### 3、检验版本
```cmd
# 检查rust版本号
rustc -V
​
​
# 检查包管理器版本号
cargo -V
```

### 4、Rust创建项目
```cmd
# 创建一般项目
cargo new <ProjectName>

# 创建(lib)扩展项目
cargo new --lib <ProjectName>
```

## Rust编码（必须的步骤！！！）
- Cargo.toml 配置文件的编辑
```toml
[package]
name = "rs_walk"            # 创建的项目名称
version = "0.1.1"           # 版本号
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]              # 代码中需要导入的库及版本，按需填写
walkdir = "2.3"
regex = "1.5"
#libc = { version = "0.2", package = "libc" }
pyo3 = { version = "0.20.2", features = ["extension-module"] }

[lib]
name = "xb_walk"            # lib.rs脚本中 #[pymodule]对应的函数名，也就是用于封装为wheel包的包名也是Python引用的包名
crate-type = ["cdylib"]     # 勿动
```
- lib.rs
```rust
# 写法参考官网，有多种方式，我使用了#[pyfn(m)]这个语法糖，简化了代码（其实就是省略了`m.add_function(wrap_pyfunction!(xb_walk, m)?)?;`与函数的返回值）
# #[pymodule] 与 #[pyfn(m)]  都是必须要写的，这个是pyO3规定好的，方便转为python可以调用的格式！
use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
use pyo3::prelude::*;

#[pymodule]
fn xb_walk(_py: Python, m: &PyModule) -> PyResult<()>{
// m.add_function(wrap_pyfunction!(xb_walk, m)?)?;
    #[pyfn(m)]
    fn walk(path: &str, filter: &str) -> Vec<String> {
        let path = PathBuf::from(path);
        let pattern = Regex::new(filter).unwrap();
        let mut matched_paths: Vec<String> = Vec::new();

        for entry in WalkDir::new(path) {
            let entry = match entry {
                Ok(e) => e,
                Err(_err) => {
                    // println!("Error while walking the directory: {}", err);
                    continue;
                }
            };

            if pattern.is_match(&entry.file_name().to_string_lossy()) {
                matched_paths.push(entry.path().to_string_lossy().to_string());
            }
        }

        matched_paths
    }
    Ok(())
}
```

## Python操作（可以放在Rust编码之后不影响！！！）
### 1、准备虚拟环境（建议使用Python自带的即可）
```cmd
# 1.进入rust项目文件夹
cd <ProjectName>

# 2.创建Python虚拟环境
python[3] -m venv .env

# 3.Linux/MacOS激活环境
source .env/bin/activate

# 3.Windows激活环境
.\.env\Scripts\activate

```
### 2、安装【matuin】库
`pip[3] install matuin`

### 3、执行构建命令
`matuin build 或者 matuin develop`

### 4、安装wheel
`pip[3] install xxx-*-.whl`

### 5、调用
```python
# 简单的案例

import xb_walk

result = xb_walk.walk('E:\\xiaobaisaf', 'xiaobaicmd.py')
print(result)

```
