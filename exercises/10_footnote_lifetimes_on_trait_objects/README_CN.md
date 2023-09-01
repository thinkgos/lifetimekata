# 附注: `trait`对象上的生命周期

在第7章中, 我们讨论了占位符(匿名)生命周期(`'_`).我们提到有三种使用它们的方式：

- 简化`impl`代码块
- 输入/返回 一个需要生命周期的类型时(`Rust建议`)
- 编写包含引用的`trait object`时

在第一个情况中, 我们看到匿名生命周期只是简化了我们需要编写的内容.
在第二种情况下, 我们看到`Rust`建议我们使用它, 但我们并*不必须*这样做 -- 生命周期推断会达到我们想要的效果.

还有一个情况, 看起来生命周期推断应该做我们想要的事情, 但实际上除非在`trait`对象的情况下使用`'_`, 否则推断并不起作用.
本章将详细介绍`trait`对象和生命周期如何一起工作.

让我们建立一个简单的例子:

```rust
trait Bool {
    fn truthiness(&self) -> bool;
}

struct True();
impl Bool for True {
    fn truthiness(&self) -> bool {
        true
    }
}

struct False();
impl Bool for False {
    fn truthiness(&self) -> bool {
        false
    }
}

fn get_bool(b: bool) -> Box<dyn Bool> {
    if b == true {
        Box::new(True())
    } else {
        Box::new(False())
    }
}

fn main() {
    let my_bool = true;
    let my_dyn = get_bool(my_bool);

    println!("{}", my_dyn.truthiness());
}
```

为了明确起见, 我们在这里创建了两个代表`true`和`false`的结构体.
它们都实现了名为`Bool`的`trait`, 该`trait`具有一个`truthiness`的函数, 该函数返回`true`或`false`.

`get_bool`函数根据传递给它的`true`或`false`返回一个包裹`Bool`的`trait`对象的`Box`.

重要的是要意识到, 由于`trait`对象可能包含或不包含引用(或者任意数量的引用).
**所有`trait`对象都具有生命周期. 即使`trait`的实现者没有包含引用, 这仍然成立.**
(附注: https://doc.rust-lang.org/reference/types/trait-object.html#trait-object-lifetime-bounds)

所以, 既然我们需要将生命周期与我们的`trait`对象关联起来, 我们可能会考虑依赖生命周期推断. 但是生命周期推断如何适用于我们的`get_bool`函数呢? 由于没有输入引用, 所以我们应该为`trait`对象指定什么输出生命周期呢? 在这里, 生命周期推断无法帮助我们.

因此, 在`RFC 599`和`RFC 1156`中, `trait`对象生命周期的规则发生了变化.这些规则相当复杂, 最好进行详细说明.
[in the reference](https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes),
在`get_bool`的情况下, 这意味着对于`dyn Bool`推断的生命周期是`'static`.

在让我们稍微改变一下例子, 使结构体包含对布尔值的引用:

```rust,ignore
trait Bool {
    fn truthiness(&self) -> bool;
}

// CHANGE 1: added &'a bool here
struct True<'a>(&'a bool);
impl<'a> Bool for True<'a> {
    fn truthiness(&self) -> bool {
        true
    }
}

// CHANGE 2: added &'a bool here
struct False<'a>(&'a bool);
impl<'a> Bool for False<'a> {
    fn truthiness(&self) -> bool {
        false
    }
}

fn get_bool(b: &bool) -> Box<dyn Bool> {
    if *b == true {
        Box::new(True(b))
    } else {
        Box::new(False(b))
    }
}

// CHANGE 3: Update the 
fn main() {
    let my_dyn = {
        let my_bool = true;
        get_bool(&my_bool)
        // my_bool is dropped here, so the trait object we're returning
        // has a dangling reference.
    };
    println!("{}", my_dyn.truthiness());
}
```

现在, 我们得到一个错误:

```sh
error: lifetime may not live long enough
  --> src/main.rs:22:5
   |
21 |   fn get_bool(b: &bool) -> Box<dyn Bool> {
   |                  - let's call the lifetime of this reference `'1`
22 | /     if *b == true {
23 | |         Box::new(True(b))
24 | |     } else {
25 | |         Box::new(False(b))
26 | |     }
   | |_____^ returning this value requires that `'1` must outlive `'static`
   |
help: to declare that the trait object captures data from argument `b`, you can add an explicit `'_` lifetime bound
   |
21 | fn get_bool(b: &bool) -> Box<dyn Bool + '_> {
   |                                       ++++

error: could not compile __ due to previous error

```

尽管生命周期推断意味着`get_bool`应该最终具有这样的签名：`fn get_bool<'elided>(b: &'elided bool) -> Box<dyn Bool + 'elided>`,
但实际情况并非如此.由于`trait`对象的特殊规则, 生命周期实际上是这样的：`fn get_bool<'elided>(b: &'elided bool) -> Box<dyn Bool + 'static>`.
这个`'static`限定是不正确的.

因此, 我们需要`'_`限定(正如这个错误消息所告诉我们的那样)来告诉`Rust`它应该使用通用的生命周期推断规则, 而不是特殊的`trait`对象规则.
