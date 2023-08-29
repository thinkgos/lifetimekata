# 类型上的生命周期

到目前为止, 我们仅讨论了适用于函数的生命周期. 不仅函数是需要明确生命周期的地方, 类型(`struct`和`enum`)也是需要生命周期.
这是因为如果类型中包含了引用, 则用户需要明确其引用的生命周期.
想象一下, 我们想将`＆str`分为两个, 并创建一个包含`start`和`end`字段的结构吗？
好吧, 我们可以写个这样的函数:

``` rust,ignore
struct SplitStr {
    start: &str,
    end: &str
}

fn split<'text, 'delim>(text: &'text str, delimiter: &'delim str) -> Option<SplitStr> {
    let (start, end) = text.split_once(delimiter)?;
    
    Some(SplitStr {
        start,
        end
    })
}

# fn main() {}
```

我们完成了! 正确吗？
好吧, 这些字符串引用可以存活多长时间.
如果我们这样调用函数该怎么办:

``` rust,ignore
# struct SplitStr {
#     start: &str,
#     end: &str
# }
# 
# fn split<'text, 'delim>(text: &'text str, delimiter: &'delim str) -> Option<SplitStr> {
#     let (start, end) = text.split_once(delimiter)?;
#     
#     Some(SplitStr {
#         start,
#         end
#     })
# }

fn main() {
    let mut parts_of_string: Option<SplitStr> = None;
    {
        let my_string = String::from("First line;Second line");
        parts_of_string = split(&my_string, ";");
    }
    
    println!("{parts_of_string:?}");
}
```

好吧, `SplitStr`结构内的引用现在悬空了, 因为它们都指向`my_string`. 但它仅存活在括号内.

因此, `Rust`要求我们指定结构内所有引用的生命周期. 这是我们修复代码的方式:

``` rust
struct SplitStr<'str_lifetime> {
    start: &'str_lifetime str,
    end: &'str_lifetime str
}

fn split<'text, 'delim>(text: &'text str, delimiter: &'delim str) -> Option<SplitStr<'text>> {
    let (start, end) = text.split_once(delimiter)?;
    
    Some(SplitStr {
        start,
        end
    })
}

# fn main() {}
```

现在, 当我们返回一个`Option<SplitStr<'text>>`, 编译器会知道结构体内部的引用必须与`'text`生命周期相同,
如果我们试图返回一个`SplitStr`, 其中引用无法活到`'text`, 那将编译错误.

## 关于枚举的注释

在枚举中, 引用的使用方式与在结构体中的使用方式完全相同. 我们在这里不详细展开说明, 因为它们是可以互换的.

``` rust
enum StringOption<'a> {
    Some(&'a str),
    None
}
# fn main() {}
```

## 两个生命周期

有时候, 结构体上会有多个生命周期. 这种情况发生在结构体内部的数据来自两个不同位置, 具有两个不同的生命周期.

以找出两个句子中独特单词的程序为例.

你可以将第一个句子设为：`"I love to swim and surf."`, 第二个句子设为：`"I love to ski and snowboard."`. 第一个句子中独特的单词是`"swim"`和`"surf"`. 第二个句子中独特的单词是`"ski"`和`"snowboard"`.

如果您说这两个句子必须共享一个生命周期, 您将迫使用户确保这两个句子来自同一个地方, 因此具有相同的生命周期. 但如果第一个句子来自在整个程序运行过程中保持打开的文件, 而第二个句子是在一个循环内部扫描出来的呢?

在这种情况下, 编译器会坚持认为扫描到的值在整个程序中都被保存, 这将不够方便.

## 练习: 结构体上的两个生命周期

在这个练习中, 我们将修改一个小程序, 该程序用于查找两个字符串之间的唯一单词. 目前, 它没有任何生命周期注解, 因此无法编译.

我们的目标是返回一个结构体, 其中包含来自第一个字符串的所有唯一单词, 以及来自第二个字符串的所有唯一单词. 它们应该具有分开的生命周期.
