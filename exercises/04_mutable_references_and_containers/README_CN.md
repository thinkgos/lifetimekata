# 可变引用和容器

可变引用与常规引用的生命周期省略规则是完全相同. 之所以独立一章关于它们, 是因为如果你有一个可变引用, 即使没有返回值, 你可能需要告诉编译器有关生命周期的信息.

例如, 让我们来看看这个例子:

``` rust,ignore
fn insert_value(my_vec: &mut Vec<&i32>, value: &i32) {
    my_vec.push(value);
}
```

我们不返回任何东西；所以生命周期无关紧要, 对吗?

很遗憾, 生命周期仍然很重要. `value`引用实际上需要与`vector`里的内容具有相同的生命周期. 如果它们的生命周期不同, 那么`vector`可能包含无效的引用. 例如, 下面这种情况下会发生什么?

``` rust,ignore
fn insert_value(my_vec: &mut Vec<&i32>, value: &i32) {
    my_vec.push(value);
}

fn main() {
    let x = 1;
    let my_vec = vec![&x];
    {
        let y = 2;
        insert_value(&mut my_vec, &y);
    }
    println!("{my_vec:?}");
}
```

上面示例中,在尝试打印`vector`时, `y`的引用是悬空的！

我们可以使用生命周期来确保两个引用存在相同的存活时间:

``` rust
fn insert_value<'vec_lifetime, 'contents_lifetime>(my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>, value: &'contents_lifetime i32) {
    my_vec.push(value)
}
fn main(){
    let mut my_vec = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1);
    insert_value(&mut my_vec, &val2);
    
    println!("{my_vec:?}");
}
```

这个签名表明存在两个生命周期:

- `'vec_lifetime`: 我们传递给函数的`vector`将需要在一定时间内存活.
- `'contents_lifetime`: `vector`里的内容需要在一定时间内存活.
重要的是, 我们插入的新`value`需要与`vector`里的内容存活一样长的时间. 如果不是这样, 最终会得到一个包含无效引用的`vector`.
- **NOTE:这里存在一个隐式生命周期,即 `where contents_lifetime: vec_lifetime`, `vector`的内容要比`vector`活的更长**

## 我们真的需要两个生命周期么?

也许您会想知道, 如果我们不提供两个生命周期, 会发生什么情况? 只提供一个生命周期能正常工作吗?

``` rust,ignore
fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
    my_vec.push(value)
}

fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1);
    insert_value(&mut my_vec, &val2);
    
    println!("{my_vec:?}");
}
```

不, 它没有正常工作. 我们出现了两个错误. 让我们看第一个错误:

```sh
error[E0499]: cannot borrow `my_vec` as mutable more than once at a time
  --> /tmp/rust.rs:11:18
   |
10 |     insert_value(&mut my_vec, &val1);
   |                  ----------- first mutable borrow occurs here
11 |     insert_value(&mut my_vec, &val2);
   |                  ^^^^^^^^^^^
   |                  |
   |                  second mutable borrow occurs here
   |                  first borrow later used here

```

这似乎有些奇怪 -- 为什么不能借用 `my_vec`?

好的, 让我们逐步了解编译器所看到的内容: `&val` 需要在 `my_vec` 存在的整个时间内保持有效:

``` rust,ignore
# fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
#     my_vec.push(value)
# }
# 
# fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // \
    insert_value(&mut my_vec, &val2); // | - &val1 needs to last this long.
                                      // |
    println!("{my_vec:?}");           // /
# }
```

而`&mut my_vec`只需要在`insert_value`函数的存活时间内有效即可.

``` rust,ignore
# fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
#     my_vec.push(value)
# }
# 
# fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // <- &mut my_vec only needs to last this long.
    insert_value(&mut my_vec, &val2); 
    
    println!("{my_vec:?}");
# }
```

但是, 我们已经告诉编译器它需要让`&val1`和`&mut my_vec`的借用拥有相同的生命周期.
因此, 编译器会延长对`&mut my_vec`的借用, 以确保它们拥有相同的生命周期: 如果让`&mut my_vec`的生命周期与`&val1`一样长, 它将在代码中拥有单一的代码区域:

``` rust,ignore
# fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
#     my_vec.push(value)
# }
# 
# fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // \
    insert_value(&mut my_vec, &val2); // | - 'one_lifetime must be this region of code.
                                      // |
    println!("{my_vec:?}");           // /
# }
```

当然, 那没问题. 但现在编译器继续到了下一行, 它发现你正在尝试再次借用 `&mut my_vec`. 前面编译器已经决定 `&mut my_vec` 必须存在直到函数结束. 所以现在, 你正在要求它创建两个可变引用... 而这是不允许的.所以编译器会抛出一个错误 -- 你不能再次借用 `&mut my_vec`.

## 为什么有两个生命周期可以修复这个错误?

在阅读这部分内容之前先思考一下 -- 为什么拥有两个生命周期可以解决这个错误?

前面, 编译器确定`&mut my_vec`和 `&val1`拥有相同的生命周期, 换句话说, 它们存活的一样长.
通过使用两个生命周期, 我们告诉编译器 `&mut my_vec` 和 `&val1` 不一定必须需要有相同的存活时间.
因此, 它找到了下面生命周期的方式:

``` rust,ignore
fn insert_value<'vec_lifetime, 'contents_lifetime>(my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>, value: &'contents_lifetime i32) {
    my_vec.push(value)
}

fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // <- 'vec_lifetime \
    insert_value(&mut my_vec, &val2); //                  | 'contents_lifetime
                                      //                  |
    println!("{my_vec:?}");           //                  /
}
```

## 练习第1部分：另一个错误

首先, 让我们来看看上一小节中出现的另一个错误:

```sh
error[E0502]: cannot borrow `my_vec` as immutable because it is also borrowed as mutable
  --> /tmp/rust.rs:13:16
   |
10 |     insert_value(&mut my_vec, &val1);
   |                  ----------- mutable borrow occurs here
...
13 |     println!("{my_vec:?}");
   |                ^^^^^^
   |                |
   |                immutable borrow occurs here
   |                mutable borrow later used here
   |
```

你能解释为什么发生此错误? 用50个或更少的词写出.

## 练习第2部分：写我们的

在这个练习中为函数添加适当的生命周期.
