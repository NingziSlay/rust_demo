# rust

[官方文档中文翻译（非官方）](https://rust-lang.tw/book-tw/title-page.html)

## Cargo

`cargo new <name>` 命令会在当前目录创建一个新项目，同时会初始化一个 git 仓库，如果不需要初始化 vcs，可以使用 `cargo new <name> --vcs=none`

由 cargo 初始化的项目中会有一个 `Cargo.toml` 文件，在 **[dependencies]** 块下定义项目的依赖，需要指定包名和版本号。

和 Golang 不同，Rust 的依赖需要手动添加到 `Cargo.toml` 中，而不是像 Golang 那样，在项目目录下使用 go get 就会自动添加依赖到 go.mod 中。

当手动添加了依赖后，可以使用 `cargo build` 或 `cargo check` 下载依赖到本地，同时会生成一个 `Cargo.lock`文件，记录着所有包的版本信息，当再次编译时 Rust 会通过 `Cargo.lock` 文件来查找依赖，确保每次编译都能使用相同的版本。

有意思的是，如果使用 `cargo update` 更新依赖时，Rust 会修改 `Cargo.lock` 文件中的包版本，而不会改变 `Cargo.toml` 中由我们自己声明的依赖版本。

## 镜像源

照着文档写 Demo 的时候，执行 `cargo build` 的时候，出现过 **Blocking waiting for file lock on package cache** 的错误，google 了一圈，有好几种解决的方式：

- rm -rf $HOME/.cargo/registry/index/* （好像并没有什么用）
- rm $HOME/.cargo/.package-cache
- 修改镜像源
- 有一个以上的 cargo build 程序正在编译同一个项目，kill 掉另一个编译任务

我在修改了镜像源以后没有再复现过这个问题，即使我把镜像源改回了官方镜像。所以无法确定哪个方法是真正有用的。

### 修改镜像源

编辑 `$HOME/.cargo/config` 文件，没有的话就创建一个

``` toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'rustcc'

# rustcc 1号源
[source.rustcc]
registry="git://crates.rustcc.com/crates.io-index"

# rustcc 2号源
[source.rustcc2]
registry="git://crates.rustcc.cn/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
```

需要使用哪个镜像，只需要修改 `replace-with` 的值为下面的 source.xxx 的 xxx 部分即可
