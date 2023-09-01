# 章节0: 引用和生命周期回顾

*这个章节是回顾,一些读者可能很熟悉.如果您已经知道引用是什么, 则可以跳过本章.*

如果您已经使用`Rust`一段时间, 您可能已经遇到过引用, 对于每个类型`T`,有两种类型的引用:

- `&T`: `T`的共享引用(通常称为共享借用). 您可以拥有任意数量的共享引用, 但它们不允许您修改它们引用的数据.
- `&mut T`: `T`的可变引用(通常称为独占借用). 您同时只能拥有一个可变引用, 但它们允许您修改它们引用的数据.

引用使得在不需要复制数据的情况下调用函数变得容易.

`Rust`引用的强大之处在于, 它始终保证引用存在的东西(即尚未被删除/释放/超出范围). 对不再存在的东西的引用被称为"dangling reference(悬空引用)", 而`Rust` 保证永远不会存在悬空引用. 因此, 这个例子将无法编译：

```rust,ignore
fn main() {
    let x_ref = {
        let x = 3;
        &x
    };
    // x_ref would now refer to `x`, but `x` is out of scope, so x_ref is dangling.
   
    println!("{}", x_ref)
}
```

大多数流行的语言(Python, Java, etc.)通过在运行时不断检查是否有对某个对象的引用, 只有在没有引用时才进行释放, 从而避免了悬空引用的问题. 这被称为"垃圾回收", 其优点是你无需考虑对象何时被释放的问题. 语言会自动为您完成这个操作. 缺点是性能方面的 -- 垃圾回收需要不时地停止您的程序, 以便语言可以扫描您拥有的每个引用.

一些语言(尤其是C和Assembly)为你提供了"指针"类型. 由于指针是内存中的原始地址, 编译器让程序员确保它们没有悬空引用. 这使得它们可以在内存受限或性能关键的环境中使用, 但不幸的是, 这意味着一个错误可能会在内存被销毁后访问内存, 导致崩溃, 甚至更糟的情况是安全问题.

`Rust`非常强大, 因为它为你提供了在运行时永远不会访问已释放内存的便利性；但为此付出的代价是在编译时需要确信你已经正确使用了引用.

## 不相信编译器的一个示例

毫无疑问, 您之前有遇到过如下错误：

```rust,ignore
fn main() {
    let mut my_reference: Option<&String> = None;

    // Starting a scope.
    {
        // my_variable created                               // \ \
        let my_variable: String = "hello".to_string();       // | |
        my_reference = Some(&my_variable);                   // | |- my_variable exists here. ('variable)
        // At the end of the scope, `my_variable` is dropped // | |
        drop(my_variable);                                   // | |
        // my variable destroyed                             // | /
    }                                                        // | - my_reference needs to exist here. ('reference)
                                                             // |
    if let Some(reference) = my_reference {                  // |
        println!("{}", reference);                           // |
    }                                                        // /
}
```

```sh
error[E0597]: `my_variable` does not live long enough
  --> bad_lifetimes.rs:7:29
   |
7  |         my_reference = Some(&my_variable);
   |                             ^^^^^^^^^^^^ borrowed value does not live long enough
8  |     }
   |     - `my_variable` dropped here while still borrowed
9  |
10 |     if let Some(reference) = my_reference {
   |                              ------------ borrow later used here

error: aborting due to previous error; 1 warning emitted

```

显然, 在这个例子中, 由于`my_variable`在`my_reference`之前超出了作用域, 所以`if let`可能会尝试访问`my_reference`, 并发现它引用的变量已经不存在了.

`Rust`表示这个变量"没有活得足够长". 它注意到"`my_variable`有可能在`my_reference`中存储该引用之前, 就已经被丢弃了"

一般的, 我们可以通过注意到这两者存在的代码区域来理解. 引用存在的代码区域比变量存在的代码区域要长.
这表明在引用存在的某部分时间内, 变量可能已经被丢弃, 因此有可能存在悬空引用.

我们称一个引用必须有效的代码区域为"生命周期". 我们可以使用语法`'name`给生命周期进行命名.
所以我们认为`'variable`是代码中引用该变量有效的区域.
另外, 可以认为`'reference`是引用可能被使用的代码区域.
我们可以正式地说`'variable`需要比`'reference`的生命周期要长.

这显然是正确的, 简言之"引用有效的代码区域必须大于引用实际可用的代码区域".
考虑相反的情况: 如果引用在引用无效的地方可用, 你将得到一些是无效的东西: 不安全的代码, 换句话说, 就是bug.

## 那么这本书是关于什么?

在某些地方, `Rust`编译器无法推导出生命周期, 需要程序员显式指定. 本书旨在帮助您改进显式生命周期的编写（例如`&'a str`）. 这将从下一章开始！

## 练习: 完成`Rustlings`生命周期的练习

如果你不确定自己是否理解以上内容, 在继续阅读之前, [完成`Rustlings`生命周期的练习](https://github.com/rust-lang/rustlings/tree/main/exercises/lifetimes).
