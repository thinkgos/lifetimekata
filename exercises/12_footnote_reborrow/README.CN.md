# 附注: reborrow

对于每个类型`T`, 有两种类型的引用[Primitive Type reference](https://doc.rust-lang.org/std/primitive.reference.html):

- `&T`: `T`的共享引用(通常称为共享借用). 您可以拥有任意数量的共享引用, 但它们不允许您修改它们引用的数据.
- `&mut T`: `T`的可变引用(通常称为独占借用). 您同时只能拥有一个可变引用, 但它们允许您修改它们引用的数据.

而`&T`实现了`Copy`, `&mut T`没有实现`Copy`.

`Rust`对`reborrow`还没有更好的文档, 更多详情参阅:

- [reborrow issue]
- [Unifying borrow and reborrow via access]

## 练习1: reborrow

```rust,editable
fn main() {
    let mut i = 42;
    let x = &mut i;
    // move and reborrow
    // let y = x; // value moved here, will cause error
    let y: &mut i32 = x; // reborrow, same as let y: &mut i32 = &mut *x;
    *y = 44;
    println!("y = {}", *y);
    *x = 45;
    println!("x = {}", *x);
}
```

```rust
fn main() {
    let mut i = 42;
    let x = &mut i;
    // reborrow, not moved, same as change_i(&mut *x);
    change_i(x);
    println!("i = {}",*x);
    *x = 45;
    println!("i = {}",*x);
}
fn change_i(i: &mut i32) {
    *i = 44;
}
```

## 练习: 引用的引用, 来自[cheats: memory lifetimes]

**NOTE: 对`&'b &'a S`, 可以把`&'a S`看成`T`(`&'a S = T`), 则有`&'b T`且有`T: 'b`约束, 则推理得出`'a: 'b`.**
**NOTE: `'a: 'b`约束表明`'a`生命周期比`'b`要长.**

### 分析下面返回一个短生命周期(`'b`)的不可变引用

```rust,editable
fn main() {}

// Return short('b') reference

struct S;

fn f1<'b, 'a>(rb: &'b &'a S) -> &'b S {
    *rb
}

fn f2<'b, 'a>(rb: &'b &'a mut S) -> &'b S {
    *rb
}

fn f3<'b, 'a>(rb: &'b mut &'a S) -> &'b S {
    *rb
}

fn f4<'b, 'a>(rb: &'b mut &'a mut S) -> &'b S {
    *rb
}
```

- [x] 看下`f1`, `&'b &'a S`有`'a: 'b`约束, `*rb`是从`&'b &'a S`解引用得到`&'c S`则有`'c: 'b`约束, 但是因为`&T`是可`Copy`的, 返回不可变引用是没有问题的, 可以编译通过.
- [x] 看下`f2`, `&'b &'a mut S`有`'a: 'b`约束, `*rb`是从`&'b &'a mut S`解引用得到`&'c mut S`则有`'c: 'b`约束, 因为`'c: 'b`, 返回值的生命周期是符合条件的, 所以一个可变的引用转成不可变引用是可以, 所以这里是能编译通过的.
- [x] 看下`f3`, 和`f1`不同的是`*rb`从`&'b mut &'a S`解引用得到`&'c S`(涉及到`reborrow`), 因为`&T`是可`Copy`的, 所以这里是能编译通过的.
- [x] 看下`f4`, 和`f2`不同的是`*rb`从`&'b mut &'a mut S`解引用得到`&'c mut S`(涉及到`reborrow`), 生命周期也符合条件, 所以这里是能编译通过的.

### 分析下面返回一个短生命周期(`'b`)的可变引用

```rust,editable
fn main() {}

// Return short('b') mutable reference

struct S;


fn f1<'b, 'a>(rb: &'b &'a S) -> &'b mut S {
    *rb
}

fn f2<'b, 'a>(rb: &'b &'a mut S) -> &'b mut S {
    *rb
}

fn f3<'b, 'a>(rb: &'b mut &'a S) -> &'b mut S {
    *rb
}

fn f4<'b, 'a>(rb: &'b mut &'a mut S) -> &'b S {
    *rb
}
```

- 看下`f1`, 不可变引用是不能转为可变引用, 所以编译不通过.
- 看下`f2`, 同`f1`, 因为`rb`是不可变引用不能将`*rb`解引用成可变引用, 所以编译不通过.
- 看下`f3`, 同`f1`, 因为`rb`是可变引用,`*rb`解引用成不可变引用, 再从不可变引用转换成可变引用是不合法的, 所以编译不通过
- [x] 看下`f4`, 可变引用的可变引用, 返回一个可变引用, 这是合法的, 现在就看生命周期是否符合要求, 生命周期推理可以看上一题`f1`的推理, `f4`是可以编译通过的

### 分析下面返回一个长生命周期(`'a`)的不可变引用

```rust,editable
fn main() {}

// Return long('a') reference

struct S;

fn f1<'b, 'a>(rb: &'b &'a S) -> &'a S {
    *rb
}

fn f2<'b, 'a>(rb: &'b &'a mut S) -> &'a S {
    *rb
}

fn f3<'b, 'a>(rb: &'b mut &'a S) -> &'a S {
    *rb
}

fn f4<'b, 'a>(rb: &'b mut &'a mut S) -> &'a S {
    *rb
}
```

- [x] 看下`f1`, `&'b &'a S`有`'a: 'b`约束, `*rb`是从`&'b &'a S`解引用得到`&'c S`则有`'c: 'b`约束, 但是因为`&T`是可`Copy`的, 返回不可变引用是没有问题的.
- 看下`f2`, `&'b &'a mut S`有`'a: 'b`,`*rb`解引用得到`&'c mut S`则有`'c: 'b`约束, 由于`&mut T`是**没有实现**`copy`的,且`'a`和`'c`无法确定关系, 所有`&'c mut S` 是无法转换成`&'a S`.
- 看下`f3`, `&'b mut &'a S`有`'a: 'b`,`*rb`解引用得到`&'c S`则有`'c: 'b`约束, 由于`&T`是**实现**`copy`的, 所以 `&'c S` 转成`&'a S`是没有问题的.
- 看下`f4`, `&'b mut &'a mut S`有`'a: 'b`,`*rb`解引用得到`&'c mut S`则有`'c: 'b`约束,由于`&mut T`是**没有实现**`copy`的, 且`'a`和`'c`无法确定关系, 所有`&'c mut S` 是无法转换成`&'a mut S`.

### 分析下面返回一个长生命周期(`'a`)的可变引用

```rust,editable
fn main() {}

// Return long('a') mutable reference

struct S;

fn f1<'b, 'a>(rb: &'b &'a S) -> &'a mut S {
    *rb
}

fn f2<'b, 'a>(rb: &'b &'a mut S) -> &'a mut S {
    *rb
}

fn f3<'b, 'a>(rb: &'b mut &'a S) -> &'a mut S {
    *rb
}

fn f4<'b, 'a>(rb: &'b mut &'a mut S) -> &'a mut S {
    *rb
}
```

- 看下`f1`, 不可变引用是不能转为可变引用, 所以编译不通过.
- 看下`f2`, 同`f1`, 因为`rb`是不可变引用不能将`*rb`解引用成可变引用, 所以编译不通过.
- 看下`f3`, 同`f1`, 因为`rb`是可变引用,`*rb`解引用成不可变引用, 再从不可变引用转换成可变引用是不合法的, 所以编译不通过
- 看下`f4`, `&'b mut &'a mut S`有`'a: 'b`,`*rb`解引用得到`&'c mut S`则有`'c: 'b`约束, 因为`'a`和`'c`无法确定关系, `&'c mut S`是无法转换成`&'a mut S`, 所以编译不通过

[reborrow issue]: https://github.com/rust-lang/reference/issues/788
[Unifying borrow and reborrow via access]: https://users.rust-lang.org/t/unifying-borrow-and-reborrow-conceptually-via-access/66065
[cheats: memory lifetimes]: https://cheats.rs/#memory-lifetimes
