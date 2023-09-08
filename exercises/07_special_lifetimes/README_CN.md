# 特殊生命周期和约束

在`Rust`中有两个特殊的生命周期, 值得讨论它们:

- `'static`
- `'_` (隐式生命周期)

## `static`的生命周期

`&'static T`代表能在整个程序运行期间存活.

你的程序中有一些内容是在整个程序运行期间存活. 其中最常见的情况是当它们作为信息捆绑在你的二进制文件中. 例如, 当你编写像这样的程序时:

``` rust
fn main() {
    let my_text = "Hello World";
}
```

文本`"Hello World"`实际上位于编译后的二进制文件中某个位置. 这意味着只要程序在运行, 对它的引用始终是有效的, 因为该文本始终存在于其中.
因此, 如果我们要谈论这种类型的文本, 我们会说它是`&'static str`.

类似地, 对于常量的任何引用也可以是`&'static`. 例如:

``` rust
const SOME_COORDINATE: (i32, i32) = (7, 4);

fn main() {
    let static_reference: &'static (i32, i32) = &SOME_COORDINATE;
}
```

## `'_`生命周期(匿名生命周期, 占位符生命周期)

隐式生命周期在`Rust`中是指让编译器自行推断导生命周期. 这种生命周期推导在以下三种情况下非常有用:

- 简化`impl`代码块
- 输入/返回 一个需要生命周期的类型时(`Rust建议`)
- 编写包含引用的`trait object`时

### 简化`impl`代码块

你正在实现一个计数器结构, 其如下:

``` rust
struct Counter<'a> {
    counter: &'a mut i32
}

impl<'a> Counter<'a> {
    fn increment(&mut self) {
        *self.counter += 1;
    }
}

fn main() {
    let mut num = 0;
    
    let mut counter = Counter { counter: &mut num };
    counter.increment();
    
    println!("{num}"); // prints 1
}
```

当你仔细观察时会注意到, `impl`块实际上并没有在任何地方使用`'a`生命周期,
因此, 我们可以通过以下方式简化代码:

``` rust,ignore
impl Counter<'_> {
    fn increment(&mut self) {
        self.counter += 1;
    }
}
```

上面的两个 `impl` 块表示的意思是相同的, 只是参数稍微少一些.

### 返回结构体和枚举

这是针对返回包含引用的结构体/枚举的情况的推荐做法, 你可以编写类似这样的代码:

``` rust

struct StrWrap<'a>(&'a str);

fn make_wrapper(string: &str) -> StrWrap {
    StrWrap(string)
}

# fn main() {}
```

但是这种语法不再推荐使用, 当您添加`#![deny(rust_2018_idioms)]`注解时, 您将会看到以下错误:

```text
error: hidden lifetime parameters in types are deprecated
 --> src/main.rs:8:34
  |
_ | fn make_wrapper(string: &str) -> StrWrap {
  |                                  ^^^^^^^ expected lifetime parameter
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
_ | #![deny(rust_2018_idioms)]
  |         ^^^^^^^^^^^^^^^^
  = note: `#[deny(elided_lifetimes_in_paths)]` implied by `#[deny(rust_2018_idioms)]`
help: indicate the anonymous lifetime
  |
_ | fn make_wrapper(string: &str) -> StrWrap<'_> {
  |                                         ++++
```

根据这个提示, 可以更清楚地看出 `StrWrap` *确实*包含一个引用, 只是编译器自己解决了这个问题.

### `trait`对象上的生命周期

详见[章节10:生命周期限制的附注](./chapter_10.md)以获取更多信息.

## 生命周期约束

生命周期约束并不被广泛使用, 因此我们在这些练习中没有为其专门划分大节.
如果你不是非常想了解细节, 可以跳过这一部分.
简单来说, 它们允许您指定一个生命周期应该比另一个生命周期更长. 要指定一个生命周期, 可以使用类似于 `where 'a: 'b` 这样的`where`子句.

引用`Rust`参考文档的说法:

> 生命周期约束可以应用于类型或其他生命周期.
> `'a: 'b`约束通常被理解为`'a`比`'b`活的更长.
> `'a: 'b` 的意思是`'a`至少和`'b`活的一样长, 因此引用`&'a ()`在与 `&'b ()` 相同情况下有效.
> ```rust,ignore
> fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
>     y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
>     let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
> }
> ```
> `T: 'a` 的意思是`T`的所有生命周期参数都要比`'a`活的长.
> 举例来说, 如果`'a`是一个无约束的生命周期参数, 那么 `i32: 'static` 和 `&'static str: 'a` 这两个条件是满足的, 但是 `Vec<&'a ()>: 'static` 则不满足.

## 练习

你已经获得代码, 其中包含许多 `'a` 和 `'b` 生命周期的用法. 所有这些生命周期可以被替换为 `'_` 或 `'static`.
你的任务是将所有的`'a`和`'b`生命周期出现替换为`'_`或`'static`, 以消除过多的生命周期声明, 并确保你的代码仍然能够编译通过.

## 已过时信息的附注

`Rust`版本指南曾经包含关于匿名生命周期的部分内容. 现在最受欢迎的谷歌搜索结果是[这篇文章](https://yegeun542.github.io/rust-edition-guide-ko/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html), 但我建议忽略它, 因为这是过时的信息.