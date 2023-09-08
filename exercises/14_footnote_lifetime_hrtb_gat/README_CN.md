# 附注: HRTB & GAT

- [RFC 387]`for <'a> ...` higher ranked trait bounds
- [RFC 1598]generic associated types
- [更好理解生命周期GAT]的一篇文章

## HRTB: higher ranked trait bounds

- `for <'a'>` 表示`for any lifetime`都会成立
- 目前 `fn`系列默认`HRTB`
- 区分生命周期的`early bound`和`late bound`.

### 示例1

以下示例是否编译通过?

```rust
fn main() {}

fn call_on_ref_zero<'a, F>(f: F) 
where
    F: Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}
```

上述示例是无法编译通过的, 这里的`'a`约束是在调用该函数就已经定下来了, `'a`的生命周期一般都要比函数要长,
但是我们的`zero`是在函数里的, `Rust`提出了`HRTB`, 我们可以改成如下

```rust,editable
fn main() {}

fn call_on_ref_zero<F>(f: F) 
where
    F: for<'a> Fn(&'a i32),
    // F: Fn(&i32)
{
    let zero = 0;
    f(&zero);
}
```

`F: for<'a> Fn(&'a i32)`的意思是对于任意的`'a`, 在`F`里面, 都是满足要求的, `Fn`可以接受任意`'a`参数,
由于`fn`系列默认都是`HRTB`, 所以我们可以省略成`F: Fn(&i32)`.

### 示例2

```rust
fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let ff = f::<'static'> as fn(); // let ff = f as fn();
    let gg = g::<'static'> as fn();
}
```

`fn f<'a>() {}`属于`late bound`的一种情况, 只有在调用`f`时才会决定`'a`的生命周期.
`fn g<'a: 'a>() {}`是`early bound`的一种情况, 在传递时就要确定`'a`. 在`where`语句,`impl`的`self`都属于`early bound`.

`early bound`: 在传递时就确定了约束
`late bound`: 在调用时才确定约束

## generic associated types

`associated type`中可以增加类型参数, 当然也可以是生命周期, 更多详情可以参考[RFC 1598]和[更好理解生命周期GAT]

[RFC 387]: https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md
[RFC 1598]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
[更好理解生命周期GAT]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
