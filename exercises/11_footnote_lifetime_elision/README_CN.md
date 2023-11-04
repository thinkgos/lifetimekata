# 附注: 译rust reference生命周期的省略规则

原文: [rust reference]  
中文: [rust reference cn]

`Rust`在许多情况下允许省略多个位置的生命周期, 但前提是编译器可以推断出合理的默认选择.

## 函数上生命周期省略规则

为了使常用模式更加符合人体工程学, 在[function item]、[function pointer]和[closure trait]的签名中可以省略生命周期参数.
以下规则用于推断出被省略的生命周期参数. 无法推断出省略的生命周期参数将导致错误. 占位符(匿名)生命周期`'_`也可以用于以相同的方式推断生命周期.
对于路径中的生命周期, 首选使用`'_`. `trait`对象的生命周期遵循不同的规则, 将在[默认`trait`对象生命周期](#默认`trait`对象生命周期)进行讨论.

- 参数中省略的每个生命周期类型参数都会(被推断)有各自独立的生命周期类型。
- 如果所有输入引用上有且只有一个生命周期, 则将该生命周期作为*所有*省略的输出生命周期的类型参数.

在方法签名中, 还有另一个规则:

- 如果接收者具有类型`&Self`或`&mut Self`，则借用的`self`的生命周期作为所有省略输出生命周期的的类型参数.

例:

```rust
# trait T {}
# trait ToCStr {}
# struct Thing<'a> {f: &'a i32}
# struct Command;
#
# trait Example {
fn print1(s: &str);                                   // elided
fn print2(s: &'_ str);                                // also elided
fn print3<'a>(s: &'a str);                            // expanded

fn debug1(lvl: usize, s: &str);                       // elided
fn debug2<'a>(lvl: usize, s: &'a str);                // expanded

fn substr1(s: &str, until: usize) -> &str;            // elided
fn substr2<'a>(s: &'a str, until: usize) -> &'a str;  // expanded

fn get_mut1(&mut self) -> &mut dyn T;                 // elided
fn get_mut2<'a>(&'a mut self) -> &'a mut dyn T;       // expanded

fn args1<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;                  // elided
fn args2<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new1(buf: &mut [u8]) -> Thing<'_>;                 // elided - preferred
fn new2(buf: &mut [u8]) -> Thing;                     // elided
fn new3<'a>(buf: &'a mut [u8]) -> Thing<'a>;          // expanded
# }

type FunPtr1 = fn(&str) -> &str;                      // elided
type FunPtr2 = for<'a> fn(&'a str) -> &'a str;        // expanded

type FunTrait1 = dyn Fn(&str) -> &str;                // elided
type FunTrait2 = dyn for<'a> Fn(&'a str) -> &'a str;  // expanded
```

```rust,compile_fail
// The following examples show situations where it is not allowed to elide the
// lifetime parameter.

# trait Example {
// Cannot infer, because there are no parameters to infer from.
fn get_str() -> &str;                                 // ILLEGAL

// Cannot infer, ambiguous if it is borrowed from the first or second parameter.
fn frob(s: &str, t: &str) -> &str;                    // ILLEGAL
# }
```

## 默认`trait`对象生命周期

[trait object]所持有引用的假定的生命周期称为其默认对象生命周期约束. 它们定义在[RFC 599]和修订的[RFC 1156].

这些默认的对象生命周期约束是在完全省略生命周期的参数时使用的, 而不是根据上面定义的生命周期通用的省略规则.
但是如果使用`'_`作为生命周期约束, 那么将遵循上面通用的省略规则

如果将`trait`对象用作泛型的类型参数, 则首先使用其包含类型来推断约束.

- 如果包含的类型有唯一的约束, 那么该约束就是默认的约束.
- 如果包含的类型有多个约束, 那么必须指定一个显式的约束.

如果以上规则都不适用, 那么将使用`trait`的生命周期约束：

- 如果`trait`使用单一生命周期`_bound_`进行定义, 则使用该约束.
- 如果在任何生命周期约束中使用了`'static`, 则使用`'static`.
- 如果`trait`没有生命周期约束, 则在表达式中推断生命周期, 并在表达式外部使用`'static`.

```rust
// For the following trait...
trait Foo { }

// These two are the same because Box<T> has no lifetime bound on T
type T1 = Box<dyn Foo>;
type T2 = Box<dyn Foo + 'static>;

// ...and so are these:
impl dyn Foo {}
impl dyn Foo + 'static {}

// ...so are these, because &'a T requires T: 'a
type T3<'a> = &'a dyn Foo;
type T4<'a> = &'a (dyn Foo + 'a);

// std::cell::Ref<'a, T> also requires T: 'a, so these are the same
type T5<'a> = std::cell::Ref<'a, dyn Foo>;
type T6<'a> = std::cell::Ref<'a, dyn Foo + 'a>;
```

```rust,compile_fail
// This is an example of an error.
# trait Foo { }
struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b> {
    f1: &'a i32,
    f2: &'b i32,
    f3: T,
}
type T7<'a, 'b> = TwoBounds<'a, 'b, dyn Foo>;
//                                  ^^^^^^^
// Error: the lifetime bound for this object type cannot be deduced from context
```

请注意, 最内层的对象确定了生命周期约束, 因此`&'a Box<dyn Foo>`仍然等同于`&'a Box<dyn Foo + 'static>`.

```rust
// For the following trait...
trait Bar<'a>: 'a { }

// ...these two are the same:
type T1<'a> = Box<dyn Bar<'a>>;
type T2<'a> = Box<dyn Bar<'a> + 'a>;

// ...and so are these:
impl<'a> dyn Bar<'a> {}
impl<'a> dyn Bar<'a> + 'a {}
```

## `'static`生命周期

除非指定明确的生命周期, 否则引用类型的常数和静态声明都具有隐式的`'static`生命周期.因此, 涉及上面`'static`的声明可以在没有标注生命周期.

```rust
#![allow(unused)]
fn main() {
// STRING: &'static str
const STRING: &str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

// BITS_N_STRINGS: BitsNStrings<'static>
const BITS_N_STRINGS: BitsNStrings<'_> = BitsNStrings {
    mybits: [1, 2],
    mystring: STRING,
};
}
```

请注意, 如果`static`或`const`项包含函数或闭包引用, 本身包含引用, 则编译器将首先尝试标准省略规则.如果它无法通过其常用的规则来解决生命周期, 那么它将出错.例如：

```rust
# #![allow(unused)]
# fn main() {
# struct Foo;
# struct Bar;
# struct Baz;
# fn somefunc(a: &Foo, b: &Bar, c: &Baz) -> usize {42}
// Resolved as `fn<'a>(&'a str) -> &'a str`.
const RESOLVED_SINGLE: fn(&str) -> &str = |x| x;

// Resolved as `Fn<'a, 'b, 'c>(&'a Foo, &'b Bar, &'c Baz) -> usize`.
const RESOLVED_MULTIPLE: &dyn Fn(&Foo, &Bar, &Baz) -> usize = &somefunc;
}
```

```rust
# #![allow(unused)]
# fn main() {
# struct Foo;
# struct Bar;
# struct Baz;
# fn somefunc<'a,'b>(a: &'a Foo, b: &'b Bar) -> &'a Baz {unimplemented!()}
// There is insufficient information to bound the return reference lifetime
// relative to the argument lifetimes, so this is an error.
const RESOLVED_STATIC: &dyn Fn(&Foo, &Bar) -> &Baz = &somefunc;
//                                            ^
// this function's return type contains a borrowed value, but the signature
// does not say whether it is borrowed from argument 1 or argument 2
}
```

[rust reference]: https://doc.rust-lang.org/reference/lifetime-elision.html
[rust reference cn]: https://rustwiki.org/zh-CN/reference/lifetime-elision.html
[function item]: https://doc.rust-lang.org/reference/types/function-item.html
[function pointer]: https://doc.rust-lang.org/reference/types/function-pointer.html
[closure trait]: https://doc.rust-lang.org/reference/types/closure.html
[trait object]: https://doc.rust-lang.org/reference/types/trait-object.html
[RFC 599]: https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
[RFC 1156]: https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md
