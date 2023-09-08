# `impl trait`生命周期省略规则规则

- [RFC 1591]`impl trait`生命周期, `impl trait` 作为返回值, 只捕获类型参数, 不捕获参数生命周期
- [RFC 2394]`async fn return impl Future`, 不同与手写 `impl trait`, `async fn` 会返回匿名 `impl Future + 参数生命周期`

## `impl trait`生命周期

`impl trait` 作为返回值, 只捕获类型参数, 不捕获参数生命周期

### 示例1

该示例是否能编译通过?

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f1<T: Foo>(t: T) -> Box<impl Foo> {
    Box::new(t)
}
```

上述示例是可以编译通过的, `impl Foo`会捕获`T`, 即使`T`是个引用, 因为`T`有指定生命周期, 它们的生命周期是一样的, 所以将会展开如下:

与`trait`对象对比, 以下示例与上述唯一不同, 返回的是一个`Box<dyn Foo>`, 是否可以编译通过?

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f2<T: Foo>(t: T) -> Box<dyn Foo> {
    Box::new(t)
}
```

上述示例是编译不通过的, 为什么? `trait`对象有一套自己的规则, 编译器展开后如下:

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f2<T: Foo>(t: T) -> Box<dyn Foo + 'static> {
    Box::new(t)
}
```

对于`T`不仅有可能是所有权类型, 也有可能是不可变引用或可变引用, 这些引用传进来时, 就包含自己的生命周期.
生命周期不匹配, 所以编译不能通过.

我们相以增加`'_`让`trait`使用通用的生命周期省略规则, 或约束`T`为`'static`.

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f2<'a, T: Foo + 'a>(t: T) -> Box<dyn Foo + 'a> {
    Box::new(t)
}

// or

fn ff2<T: Foo + 'static>(t: T) -> Box<dyn Foo + 'static> {
    Box::new(t)
}
```

综上, `impl trait`会捕获`T`, 如果`T`有生命周期, 则`impl trait`将使用这个生命周期.
而`trait`对象在没有明确标注下, 拥有自己一套规则, 可以使用`'_`使用通用的生命周期省略规则或明确标注生命周期.

### 示例2

该示例是否能编译通过?

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f3(s: &str) -> Box<impl Foo> {
    Box::new(t)
}
```

以上示例是不可编译通过的, 因为`impl trait`, 只捕获类型参数, 不捕获参数生命周期, 编译器展开后如下:

```rust
// fn f3<'a>(s: &'a str) -> Box<impl Foo + 'a> {
// fn f3(s: &str) -> Box<impl Foo + '_> {
fn f3<'a>(s: &'a str) -> Box<impl Foo> {
    Box::new(t)
}
```

我们可以对`Box<impl Foo>`补充生命周期成这样: `fn f3<'a>(s: &'a str) -> Box<impl Foo + 'a>`或`fn f3(s: &str) -> Box<impl Foo + '_>`,这样就可以编译通过了.

与`trait`对象对比, 以下示例与上述唯一不同, 返回的是一个`Box<dyn Foo>`, 是否可以编译通过?

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f4(s: &str) -> Box<dyn Foo> {
    Box::new(t)
}
```

上述示例是编译不通过的, 为什么? `trait`对象有一套自己的规则, 编译器展开后如下:

```rust
trait Foo {}
impl Foo for &'_ str {}

fn f4(s: &str) -> Box<dyn Foo + 'static> {
    Box::new(t)
}
```

我们相以增加`'_`让`trait`使用通用的生命周期省略规则, 或约束`&str`为`&'static str`.
即使
`fn f4(s: &str) -> Box<dyn Foo + '_>`或`fn f4(s: &static str) -> Box<dyn Foo + 'static>`.

综上, `impl trait`只捕获类型参数, 不捕获参数生命周期, 而`trait`对象在没有明确标注下, 拥有自己一套规则, 可以使用`'_`使用通用的生命周期省略规则或明确标注生命周期.

## `async fn return impl Future`

不同与手写 `impl trait`, `async fn` 会返回匿名 `impl Future + 参数生命周期`

### 示例3

```rust,editable

fn main() {
    let future;
    {
        let s = String::from("any");
        future = f1(&s);
        // future = f2(&s);
    }
    let another_future = future;
}

fn f1(s: &str) -> impl Future<Output=()> {
    async move {
        //  println!("{}", s)
        ()
    }
}
async fn f2(s: &str) -> () {
    ()
}
fn f3<'a, 'b>(s1: &str, s2: &str) -> impl Future<Output=() + 'a> {
    println!("{}", s2)
    async move {
        println!("{}", s1)
        ()
    }
}
```

[RFC 1591]: https://github.com/rust-lang/rfcs/blob/master/text/1951-expand-impl-trait.md
[RFC 2394]: https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md
