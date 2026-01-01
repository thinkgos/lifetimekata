# 总结生命周期规则

对`&'b &'a S`, 可以把`&'a S`看成`T`(`&'a S = T`), 则有`&'b T`且有`T: 'b`的约束, 则推理得出`'a: 'b`. `'a: 'b`约束表明`'a`生命周期比`'b`要长.

## 1 函数上的生命周期

- 参数中每个省略的生命周期类型参数都会有各自独立的生命周期类型。
- 如果所有输入引用上有且只有一个生命周期, 则将该生命周期作为所有省略的输出生命周期的类型参数.
- 如果接收者具有类型`&Self`或`&mut Self`，则借用的`self`的生命周期作为所有省略输出生命周期的的类型参数.

相关:

- [章节3: 函数上的生命周期省略规则(lifetimekata)](../03_lifetime_elision/README_CN.md)
- [章节11: 生命周期省略规则(lifetimekata)](../11_footnote_lifetime_elision/README_CN.md)
- [Rust Reference: Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)

## 2 类型上的生命周期(struct和enum)和类型impl块上的生命周期

类型(struct和enum)也是需要生命周期. 这是因为如果类型中包含了引用, 则用户需要明确其引用的生命周期.
当struct或enum带有生命周期时, impl块的工作方式也会略有变化.

相关:

- [章节5: 类型上的生命周期(lifetimekata)](../05_lifetimes_on_types/README_CN.md)
- [章节6: impl块的生命周期(lifetimekata)](../06_lifetimes_on_impls/README_CN.md)

## 3 特殊生命周期(`'static`和`'_`)

- `'static`代表能在整个程序运行期间存活.
- `'_` (隐式生命周期, 占位符生命周期)代表让编译器自行推断的生命周期.在以下三种情况下非常有用:
  - 简化`impl`代码块
  - 输入/返回 一个需要生命周期的类型时(Rust建议)
  - 编写包含引用的`trait object`时, 让编译器采用函数上的生命周期省略规则进行推断.

相关:

- [章节7: 特殊生命周期(lifetimekata)](../07_special_lifetimes/README_CN.md)

## 4 可变引用和容器的生命周期

可变引用与常规引用的生命周期省略规则是完全相同, 你有一个可变引用, 即使没有返回值, 你可能需要告诉编译器有关生命周期的信息.

相关:

- [章节4: 可变引用和容器的生命周期(lifetimekata)](../04_mutable_references_and_containers/README_CN.md)

## 5 reborrow 重借用

相关:

- [章节12: 附注: reborrow(lifetimekata)](../12_footnote_reborrow/README_CN.md)
- [reborrow issue](https://github.com/rust-lang/reference/issues/788)
- [Unifying borrow and reborrow via access](https://users.rust-lang.org/t/unifying-borrow-and-reborrow-conceptually-via-access/66065)

## 6 `impl trait`的生命周期

- [RFC 1591](https://github.com/rust-lang/rfcs/blob/master/text/1951-expand-impl-trait.md)`impl trait`生命周期, `impl trait` 作为返回值, 只捕获类型参数, 不捕获参数生命周期
- [RFC 2394](https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md)`async fn return impl Future`, 不同于手写 `impl trait`, `async fn` 会返回匿名 `impl Future + 参数生命周期`

相关:

- [章节13: 附注: impl trait的生命周期(lifetimekata)](../13_footnote_lifetime_on_impl_trait/README_CN.md)

## 7. `trait` 生命周期约束

[trait object](https://doc.rust-lang.org/reference/types/trait-object.html)所持有引用的假定的生命周期称为其默认对象生命周期约束. 它们定义在[RFC 599](https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md)和修订的[RFC 1156](https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md).

这些默认的对象生命周期约束是在完全省略生命周期的参数时使用的, 而不是根据上面定义的生命周期通用的省略规则.
但是如果使用`'_`作为生命周期约束, 那么将遵循上面通用的省略规则

如果`trait`对象被用作泛型的类型参数, 则首先使用其包含类型来尝试推断约束.

- 如果包含的类型有唯一的约束, 那么该约束就是默认的约束.
- 如果包含的类型有多个约束, 那么必须指定一个显式的约束.

如果这些规则都不适用, 则使用`trait`对象的生命周期约束：

- 如果`trait`中定义了单一生命周期约束, 则使用该约束.
- 如果在任何生命周期约束中使用了`'static`, 则使用`'static`.
- 如果`trait`没有生命周期约束, 则在表达式中推断生命周期, 并在表达式外部使用`'static`.

相关:

- [Rust Reference: Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [章节10: trait对象生命周期约束](../10_footnote_lifetimes_on_trait_objects/README_CN.md)
- [章节11: 生命周期省略规则(lifetimekata)](../11_footnote_lifetime_elision/README_CN.md)

## 8. HRTB & GAT

相关:

- [章节14: 附注: HRTB & GAT(lifetimekata)](../14_footnote_lifetime_hrtb_gat/README_CN.md)
- [RFC 387](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md)
- [RFC 1598](https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md)
- [更好理解生命周期GAT](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats)

