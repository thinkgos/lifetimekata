# 统一访问借用和重借用

原文: [Unifying borrow and reborrow via access]

- [reborrow issue](https://github.com/rust-lang/reference/issues/788)
- [cheats: memory lifetimes](https://cheats.rs/#memory-lifetimes)

`Reborrow`作为`Rust`类型系统的一个特性, 常常被认为是"编译器的魔法", [文档不全](https://github.com/rust-lang/reference/issues/788). 但在`Rust`的整个所有权和借用逻辑中却扮演着非常重要的角色. 因此, 这篇简短的教程将通过一个统一的概念: 对访问权限的操作, 讲述我对`reborrow`和`borrow`的理解.

首先, 我想说明为什么`Rust`的借用规则不能很好地解释`reborrow`. 请看下面的程序:

```rust
let mut i = 1; // forget i32 being copyable for a moment
let m1 = &mut i;
let m2 = &mut *m1;
*m2 = 2;
*m1 = 3;
```

上述代码可以编译, 但当我们赋值给`m2`时, 尽管`m1`和`m2`都借用了`i`, 它们仍然没有完全脱离作用域.

当然, 如果我们考虑到NLL的非连续生命周期, 这种行为就不会违反"只有一个可变借用"原则: 在`*m2 = 2`行之前和之后`m1`都是活的, 但在这一行中, `m1`却不是. 但这种简单的解释并不能说明为什么交换两个赋值会导致类型错误. 为了涵盖这种行为, 我们还需要一条额外的规则, 即两个可变借用的生命周期不得"重叠". 我们要求其中一个借用的作用域包含在另一个借用的作用域中.

不过, 上述规则并不能完全说明一切. 如果我们将`m2`定义为`&mut i`, 即直接借用`i`而不是借用`m1`, 上述代码将无法编译. 这一次, 我们必须添加`reborrow`规则.

既然我们已经解释了上面的所有行为, 那么就我个人而言, 这种解释绝不令人满意. 无重叠"规则和借用规则似乎是人为的, 这可能是造成借用给人"编译器魔法"印象的原因. `Rust`的类型系统非常复杂, 不可避免地会有魔法存在. 但如果在这种特殊情况下有更好的解释呢? 这就是我要在下文中提出的: 通过"访问"的概念, 对借用和再借用提出一种(不知道是否新颖)统一的看法.

虽然这不是一个官方术语, 但访问的概念在`Rust`中以一种非常明显的方式出现(假设`T`是一个非复制类型):

- `T`类型的值对持有`T`拥有所有权和(= read + write + deallocate)访问权限.
- `&mut T`类型的值对持有`T`具有(唯一的)可变借用(= read + write)访问地址的权限.
- `&T`类型的值对持有`T`具有(共享的)不可变借用(= read)访问地址的权限.

我们可以用访问的语言来重新表述借用的过程. 当我们从所有者处借用时, 我们实际上是在借用对所有者地址的访问权限, 可以是可变的, 也可以是不可变的. 当所有者被移动、分配或杀死时, 它的任何借用都不能存活, 因为我们需要收回所有者的访问权限才能执行这些操作.

那么`reborrow`呢? 从访问权限的角度来看, `reborrow`只不过是对非所有者访问权限的操作而已. 让我们通过一个简单的案例分析来说明这一点:

- 可变的重借用一个可变借用: 被借用者持有从别处借来的某个地址的`read-write`权限, 现在我们暂时拿走该权限, 并将其传给借用者
- 不变地重借用一个可变借用: 与上述情况类似, 但我们不是将借用的读写权限直接传递下去, 而是将其转换成许多读取权限.
- 重借用一个不变的借用: 类似

因此, 从访问的角度来看，借用和重借用本质上是同一回事: 操纵和借用访问权限。为了统一借用和重借用，我们只需要对著名的"一个可变或多个不变"规则进行一些修改: 我们将其推广到访问借用图, 如下:

![ownership](https://global.discourse-cdn.com/business5/uploads/rust_lang/original/3X/0/8/084028dedf986cb14cd26af0ff71874f5bb8b8ca.png)

箭头表示借用和操作访问权限的有效方向, "所有者"一栏缺少一个自循环箭头, 因为`Rust`要求内存管理的所有权是唯一的, 也就是说, 去分配访问权限不能被借用.
现在, 让我们用访问语言来重新表述 `Rust` 的原理:

- 如上图所示, 值的存取可以是借用的.
- 当一个值被移动、赋值或退出作用域时, 它必须收回其所有访问权限

让我们运用这些重新表述的原则, 看看它是否能很好地解释本篇文章开头的例子:

```rust
let mut i = 1; // forget i32 being copyable for a moment
let m1 = &mut i; // m1 borrows its access from i
let m2 = &mut *m1; // m2 (re)borrows its access from m1
*m2 = 2; // m2 using its access for writing
*m1 = 3; // We are assigning to m1 here, so m1 must take back its access.
         // Hence m2 must drop out of scope here.
// Now m1 is still usable, but m2 is not
```

如果我们交换这两个赋值:

```rust
let mut i = 1; // forget i32 being copyable for a moment
let m1 = &mut i; // m1 borrows its access from i
let m2 = &mut *m1; // m2 (re)borrows its access from m1
*m1 = 3; // We are assigning to m1 here, so m1 must take back its access.
         // Hence m2 must drop out of scope here.
*m2 = 2; // Now m2 already drops out of scope, type error!
```

如果我们设置`m2 = &mut i`:

```rust
let mut i = 1; // forget i32 being copyable for a moment
let m1 = &mut i; // m1 borrows its access from i
let m2 = &mut i; // m2 borrows from i directly
                 // To afford this new borrow, i must take its access back
                 // Hence m1 must drop out of scope
*m2 = 2; // m2 using its access for writing
*m1 = 3; // m1 already drops out of scope, type error!
```

耶, 新的原则完美运行! 最后, 如果你知道用于测试不安全代码的[`stacked borrow`](https://www.ralfj.de/blog/2018/08/07/stacked-borrows.html#:~:text=Stacked%20Borrows%3A%20An%20Aliasing%20Model%20For%20Rust%20In,when%20to%20perform%20which%20kinds%20of%20memory%20accesses.)模型, 上面的访问解释也可以推导出堆叠借用的原理.  假设我们有一个可变(再)借用链:

```owner -> m1 -> m2 -> ... -> mn```

如果我们现在赋值给`m1`, `m1`必须从`m2`处收回它的访问权. 因此, `m2`必须退出作用域, 但这又要求`m2`收回它的访问权. 最终, 从`m2`到`mn`的所有内容都必须被回收, 或者在堆栈借用模型中被"弹出堆栈", 以便分配给`m1`.

[Unifying borrow and reborrow via access]: https://users.rust-lang.org/t/unifying-borrow-and-reborrow-conceptually-via-access/66065
