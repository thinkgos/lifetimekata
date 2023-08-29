# 什么是生命周期标注?

在上一节中, 我们讨论了单个函数中生命周期的概念. 在所有这些示例中, 根据大括号能很清楚变量或引用存在于代码的哪个区域.
生命周期标注用于帮助编译器理解那些无法依赖于括号作用域的情况(如跨函数边界和在`struct`和`enum`内部)
理解生命周期标注的最好方法是首先理解为什么我们需要它们. 让我们通过一些示例来看看为什么它们存在:

详述生命周期的最简单示例函数, 该函数返回两个整数中较大者的引用.

``` rust,ignore
fn max_of_refs(a: &i32, b: &i32) -> &i32 {
    if *a > *b {
        a
    } else {
        b
    }
}
```

假设我们这样调用这个函数:

``` rust,ignore
fn complex_function(a: &i32) -> &i32 {
    let b = 2;
    max_of_refs(a, &b)
}

fn main() {
    let a = 1;
    let my_num = complex_function(&a);
    println!("{my_num}");
}
```

如果你思考这个例子, 你会发现`my_num`将会引用`complex_function`中的一个变量(该变量不再存在).
换句话说, `complex_function`的返回值的生命周期会比`b`的生命周期更长.
现在, 你可能会说, "但是难道编译器不能在运行时发现这个程序显然行不通吗?"
嗯, 因为我们在使用常量, 是的, 编译器很可能能够发现这个程序行不通.
但是如果我们使用`let a = rand::rand()`或者`let b = read_number_from_stdin()`呢?
对于编译器来说, 很难判断这个引用是否应该是有效的.

## 好的, 为什么我们不能就禁止那种情况呢?

你接下来的想法可能是: "好吧, 这种类型的所有引用肯定都是不可靠的;让我们禁止它们."
明确规定这项禁止是值得的. 最简单的禁用可能是"在函数参数中不允许使用引用",
但这可能会有些过于严格(并且对`Rust`的实用性完全具有破坏性).

一个更明智的禁止, 可以涵盖这种情况的内容如下: "任何具有多个引用输入的函数不得返回引用(或包含引用的内容).", 这样可以避免我们所见问题, 即不清楚引用来自何处. 这将禁止上述示例.

但是, 这符合人体工程学吗?如果你想要一个像这样的函数怎么办:

``` rust,ignore
fn only_if_greater(number: &i32, greater_than: &i32) -> Option<&i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}
```

无论以何种方式调用此函数, 我们始终知道, 如果我们的返回值是 `Some`, 它引用的是`number`. 它永远不会返回一个指向 `greater_than` 的引用.

一个更有趣的例子是 `split` 函数, 它接收一个字符串, 并返回由该字符串分割的一组子字符串片段, 通过另一个字符串进行分割.

``` rust,ignore
fn split(text: &str, delimiter: &str) -> Vec<&str> {
    let mut last_split = 0;
    let mut matches: Vec<&str> = vec![];
    for i in 0..text.len() {
        if i < last_split {
            continue
        }
        if text[i..].starts_with(delimiter) {
            matches.push(&text[last_split..i]);
            last_split = i + delimiter.len(); 
        }
    }
    if last_split < text.len() {
        matches.push(&text[last_split..]);
    }
    
    matches
}
```

无论如何调用此函数, 它总是会从`text`返回一组切片, 而不会从`delimiter`返回.

## 唉, 但是编译器不能自己解决这个问题吗?

到了这一步, 你可能已经注意到`matches.push`只会对`text`切片调用,
因此, 你可能会期望编译器在这种情况下能够自动推断生命周期.

在简单情况下, 这可能是可以的. 但是你的编译器可能会无法推断出生命周期. 或者在经过6个月后, 它可能能成功地推断出生命周期.

因此, 编译器需要更多信息. 这些信息是通过生命周期标注提供的, 在我们详细讨论它们之前, 这里有一个练习, 希望能在我们处理语法之前加深对这些概念的理解.

## 练习: 辨别哪些程序能够运行, 哪些会出错

在不使用任何生命周期语法的情况下, 回答每个代码示例的以下问题.

1. 这些输入是引用?这个函数可能会返回什么?
2. 哪些例子可能会出现悬空引用?

NOTE: **这些代码示例无法编译; 您需要阅读并思考它们**.

一旦你决定好了你的答案, 代码块右上角的"眼睛"按钮将显示出正确答案.

``` rust,ignore

# // a is the only input reference.
# // the only thing the function can return is a
fn identity(a: &i32) -> &i32 {
    a
}

# // This does not have any dangling references.
fn example_1() {
    let x = 4;
    let x_ref = identity(&x);
    assert_eq!(*x_ref, 4);
}

# // This is always going to cause a dangling reference.
fn example_2() {
    let mut x_ref: Option<&i32> = None;
    {
        let x = 7;
        x_ref = Some(identity(&x));
    }
    assert_eq!(*x_ref.unwrap(), 7);
}
```

``` rust,ignore
# // the contents of `opt` and `otherwise` are both references
# // either of them could be returned.
fn option_or(opt: Option<&i32>, otherwise: &i32) -> &i32 {
    opt.unwrap_or(otherwise)
}

# // No possibility for a dangling reference here.
fn example_1() {
    let x = 8;
    let y = 10;
    let my_number = Some(&x);
    assert_eq!(&x, option_or(my_number, &y));
}

# // This is always a dangling reference.
fn example_2() {
    let answer = {
        let y = 4;
        option_or(None, &y)
    };
    assert_eq!(answer, &4);
}

# // This is never a dangling reference.
fn example_3() {
    let y = 4;
    let answer = {
        option_or(None, &y)
    };
    assert_eq!(answer, &4);
}

# // This is always a dangling reference.
fn example_4() {
    let y = 4;
    let answer = {
        let x = 7;
        option_or(Some(&x), &y)
    };
    assert_eq!(answer, &7);
}
```
