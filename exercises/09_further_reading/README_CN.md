# `Rust`引用的延伸问读

有关生命周期的最佳信息来源是`Rust Reference`和`Rustonomicon`.如果您需要掌握关于生命周期的高级知识来完成项目, 参考中将包含这些内容. 但是, 大多数情况下, 如果您觉得需要了解生命周期的复杂内容, 您可能会发现有更简单的替代方案.

- [The Rust Reference (Lifetime Elision)](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [The Rust Reference (In General)](https://doc.rust-lang.org/reference/)
- [The Rustonomicon (Lifetimes)](https://doc.rust-lang.org/nomicon/lifetimes.html)

## 其他有用的生命周期内容

- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
- [Crust of Rust: Lifetime Annotations](https://www.youtube.com/watch?v=rAl-9HwD858)

## Variance and Subtyping

这份指南完全没有涵盖`"variance"`这个主题, 而`"variance"`是有关生命周期如何彼此替代的内容。虽然从理论上讲`variance`很重要, 但在日常理解生命周期的过程中并不常用, 因此在这本书中没有包含相关内容。

您可以阅读[the Rustonomicon (subtyping)](https://doc.rust-lang.org/nomicon/subtyping.html)以获取更多信息.

## 难题1: 为什么这个程序不工作?

如果您有兴趣解决一个非常具有挑战性的练习, 以测试您对生命周期和泛型的理解, 以下练习可能会很有趣.

这应该是练习5中实施代码的另一种方式.不幸的是, 这并不能正常工作。这个问题让这本书的作者花了20分钟的时间才解决(在写了五章内容之后), 所以他向你发起挑战, 希望你能做得更好！

```rust,ignore
use std::collections::HashSet;

struct Difference<'first, 'second> {
    first_only: Vec<&'first str>,
    second_only: Vec<&'second str>
}

fn find_difference<'fst, 'snd>(sentence1: &'fst str, sentence2: &'snd str) -> Difference<'fst, 'snd> {
    let sentence_1_words: HashSet<&str> = sentence1.split(" ").collect();
    let sentence_2_words: HashSet<&str> = sentence2.split(" ").collect();

    Difference {
        first_only: (&sentence_1_words - &sentence_2_words).into_iter().collect(),
        second_only: (&sentence_2_words - &sentence_1_words).into_iter().collect(),
    }

}

fn main() {
    let first_sentence = String::from("I love the surf and the sand.");
    let second_sentence = String::from("I hate the surf and the sand.");

    let first_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&first_sentence, &third_sentence);
        diff.first_only
    };

    assert_eq!(first_only, vec!["hate", "surf"]);

    let second_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&third_sentence, &second_sentence);
        diff.second_only
    };

    assert_eq!(second_only, vec!["snow"]);
}
```

有关此问题的更多信息, 请阅读[this Rust issue](https://github.com/rust-lang/rust/issues/73788).
