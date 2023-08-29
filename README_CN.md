# LifetimeKata

[英文](https://tfpk.github.io/lifetimekata/)

欢迎来到LifetimeKata, 一组可用于提高您对`Rust`生命周期的理解的练习.
许多任务涉及编写编译代码, 有些任务还涉及创建特定错误.

您应该按顺序完成kata, 因为它们的难度是逐渐增加, 并且依赖于之前的kata.

## 准备开始

克隆仓库:

``` sh
git clone https://www.github.com/thinkgos/lifetimekata
```

大多数练习分两步进行：

``` sh
cargo build --package ex04
```

然后:

``` sh
cargo test --package ex04
```

或者:

``` sh
cargo run --package ex04
```

取决于它是`binary`还是`library`。
