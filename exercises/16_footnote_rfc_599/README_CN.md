# 概要

为对象类型添加默认生命周期绑定，这样就不必再写`Box<Trait+'static>`或`&'a (Trait+'a)`之类的内容。
默认值将根据对象类型出现的上下文而定。通常情况下，出现在引用下面的对象类型的生命周期为其出现的最内层引用的生命周期，
否则默认为 `"static"`。但是，带有 `T:'a` 注解的用户自定义类型会覆盖默认值。

例:

- `&'a &'b SomeTrait` -> `&'a &'b (SomeTrait+'b)`
- `&'a Box<SomeTrait>` ->  `&'a Box<SomeTrait+'a>`
- `Box<SomeTrait>` ->  `Box<SomeTrait+'static>`
- `Rc<SomeTrait>` ->  `Rc<SomeTrait+'static>`
- `std::cell::Ref<'a, SomeTrait>` ->  `std::cell::Ref<'a, SomeTrait+'a>`

如果生命周期界限是明确给出的，或者可以从相关特征中推断出来，那么这种情况自然不受影响。

## 动机

### 目前情况

如[RFC 34]所述，对象类型只有一个生命周期界限。有时，可以根据所涉及的特征推断出这个约束。然而，通常情况下是无法推断的，在这种情况下，必须明确给出生命周期约束。以下是一些会报错的示例：

```rust
struct SomeStruct {
    object: Box<Writer>, // <-- ERROR No lifetime bound can be inferred.
}

struct AnotherStruct<'a> {
    callback: &'a Fn(),  // <-- ERROR No lifetime bound can be inferred.
}
```

这类错误经常给新用户造成困惑（部分原因是错误信息不准确）。为避免出现错误，这些示例应编写如下：

```rust
struct SomeStruct {
    object: Box<Writer+'static>,
}

struct AnotherStruct<'a> {
    callback: &'a (Fn()+'a),
}
```

自引入以来，人们一直希望这种完全显式的符号在常见情况下更加简洁。在实践中，对象的边界几乎总是与对象出现的上下文紧密相关：例如，很少有不以 "static "或 "Send "为边界的方框对象类型（如 `Box<Trait+'a>`）。同样，对象引用本身也有明显的约束（例如，`&'a (Trait+'b)`），这种情况也很罕见。这并不是说这些情况从未出现过；正如我们将在下文看到的，这两种情况在实践中确实都会出现，但它们相对来说并不常见（事实上，从来没有一个很好的理由去做 `&'a (Trait+'b)`，尽管可能有一个理由让 `&'a mut (Trait+'b)`；详见 "[详细设计]"）。

`RFC 458` 将 `Send` 特性与 `'static` 绑定断开，这使得对速记的需求变得更加迫切。这意味着现在写成 `Box<Foo+Send>` 的对象类型必须写成 `Box<Foo+Send+'static>`。

```rust
trait Message : Send { }
Box<Message> // ERROR: 'static no longer inferred from `Send` supertrait
Box<Writer+Send> // ERROR: 'static no longer inferred from `Send` bound
```

### 建议规则

本 `RFC` 建议使用对象类型出现的上下文来推导合理的默认值。具体来说，默认值以 `'static` 开头。类型构造函数（如 `&` 或用户定义的结构体）可以改变其类型参数的默认值，如下所示：

- 默认值以 `'static` 开头。
- `&'a X` 和 `&'a mut X` 将 `X` 内对象边界的默认值改为 `'a`。
- 用户定义类型（如 `SomeType<X>`）的默认值由定义在 `SomeType` 上的 `where` 约束驱动，详见下一节。高层的想法是，如果 `SomeType` 上的 `where`约束 表明 `X` 的借用寿命为`'a`，那么 `X`中出现的对象的默认值就会变成`'a`。

这些规则的基本原理是，不包含在引用中的对象默认为 `'static`，否则默认为引用的生命周期。这几乎总是你想要的。下面的统计数据显示了三个 Rust 项目中特质引用的频率，可以作为证据。最后一列显示了建议规则所能正确预测的使用百分比。

[RFC 34]: https://github.com/rust-lang/rfcs/blob/master/text/0034-bounded-type-parameters.md
