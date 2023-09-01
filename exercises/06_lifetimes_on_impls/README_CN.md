# `impl`的生命周期

当`struct`或`enum`带有生命周期时, `impl`块的工作方式也会略有变化.

例如, 假设我们想创建一个`struct`, 使用户能够在上面进行迭代. 你可以像这样:

``` rust,ignore
// First, the struct:

/// This struct keeps track of where we're up to in the string.
struct WordIterator<'s> {
    position: usize,
    string: &'s str
}

impl WordIterator {
    /// Creates a new WordIterator based on a string.
    fn new(string: &str) -> WordIterator {
        WordIterator {
            position: 0,
            string
        }
    }
    
    /// Gives the next word. `None` if there aren't any words left.
    fn next_word(&mut self) -> Option<&str> {
        let start_of_word = &self.string[self.position..];
        let index_of_next_space = start_of_word.find(' ').unwrap_or(start_of_word.len());
        if start_of_word.len() != 0 {
            self.position += index_of_next_space + 1;
            Some(&start_of_word[..index_of_next_space]) 
        } else {
            None
        }
    }
}

fn main() {
    let text = String::from("Twas brillig, and the slithy toves // Did gyre and gimble in the wabe: // All mimsy were the borogoves, // And the mome raths outgrabe. ");
    let mut word_iterator = WordIterator::new(&text);
    
    assert_eq!(word_iterator.next_word(), Some("Twas"));
    assert_eq!(word_iterator.next_word(), Some("brillig,"));
    
}
```

在定义我们的`WordIterator`结构体时, 我们要求必须指定一个生命周期. 然而, 在接下来的编写的`impl` 块中, 我们并没有指定生命周期. `Rust`要求我们这样做. 我们的这个做法是告诉 `Rust`一个生命周期, 然后将该生命周期应用于我们的结构体. 让我们看看具体怎么做:

``` rust,ignore
impl<'lifetime> for WordIterator<'lifetime> {
    // ...
}
```

值得注意的是, 我们将这个过程分为两个部分 -- `impl<'lifetime>` 定义了一个生命周期 `'lifetime`. 它并不对这个生命周期是什么做出任何承诺, 只是声明它的存在.  
然后`WordIterator<'lifetime>`使用我们创建的生命周期, 并且表示在 `WordIterator` 中的引用必须在 `lifetime` 内有效".  
现在, 在`impl`块的任何地方, 我们可以选择使用那个生命周期. 任何我们用`'lifetime'`标注的引用，必须与任何其他用`'lifetime'`标注的引用具有相同的生命周期。

``` rust,ignore
# /// This struct keeps track of where we're up to in the string.
# struct WordIterator<'s> {
#     position: usize,
#     string: &'s str
# }

impl<'lifetime> WordIterator<'lifetime> {
    /// Creates a new WordIterator based on a string.
    fn new(string: &'lifetime str) -> WordIterator<'lifetime> {
        WordIterator {
            position: 0,
            string
        }
    }
    
    /// Gives the next word. `None` if there aren't any words left.
    fn next_word(&mut self) -> Option<&str> {
        let start_of_word = &self.string[self.position..];
        let index_of_next_space = start_of_word.find(' ').unwrap_or(start_of_word.len());
        if start_of_word.len() != 0 {
            self.position += index_of_next_space + 1;
            Some(&start_of_word[..index_of_next_space]) 
        } else {
            None
        }
    }
}

# fn main() {
#     let text = String::from("Twas brillig, and the slithy toves // Did gyre and gimble in the wabe: // All mimsy were the borogoves, // And the mome raths outgrabe. ");
#     let mut word_iterator = WordIterator::new(&text);
#     
#     assert_eq!(word_iterator.next_word(), Some("Twas"));
#     assert_eq!(word_iterator.next_word(), Some("brillig,"));
#     
# }

```

## 生命周期的省略规则回顾

我们前面讨论了生命周期省略的两个规则. 他们是

- 每个没有提供输入生命周期的参数都将分配其各自独立的生命周期
- 如果所有输入引用上有且只有一个生命周期, 那么那个生命周期就会被赋予*每个*输出.

既然我们已经看过具有生命周期的`impl`块, 让我们再讨论一个问题:

- 如果存在多个输入生命周期, 但其中一个是`&self`或`&mut self`, 则借用的`self`的生命周期将分配给所有省略生命周期的输出.

这意味着, 即使你在论证中使用了多个引用, `Rust`也会假定你返回的任何引用都来自于`self`, 而不是其他引用.

## 练习

在下面的代码中, 我们使用了`'borrow`生命周期来为函数进行了标注, 而不仅仅是`'lifetime`生命周期.
`'borrow`生命周期仅在此函数内部存在, 并且仅影响其参数和返回值的借用. 正如我们之前所看到的, `'lifetime`值也约束了结构体内部字符串的生命周期.

有四种方式可以实现这段代码. 描述一下每种实现方式的效果.

具体来说:

- 它们能编译么?
- 他们中有没有任何一个与另一个完全相同？
- 有没有任何情况下他们的生命周期不够通用？
- 哪一个写法会比较正确？

### 例1

``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. It's the exact same as Example 4.
#    /// This function is problematic because the next word lives as long
#    /// as your borrow of the iterator. In order to get the next word, you
#    /// must drop all references to the current one.
    fn next_word<'borrow>(&'borrow mut self) -> Option<&'borrow str> {
        // ...
    }
```

### 例2

``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. It's the exact same as Example 3.
    fn next_word<'borrow>(&'borrow mut self) -> Option<&'lifetime str> {
        // ...
    }
```

### 例3

``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. It's probably the "most" correct, because it's the shortest
#    /// to write, but also ensures you can retain the returned strings, even if
#    /// you call this function multiple times.
    fn next_word(&mut self) -> Option<&'lifetime str> {
        // ...
    }
```

### 例4

``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. If expanded, it would be the same as Example 1.
    fn next_word(&mut self) -> Option<&str> {
        // ...
    }
```
