# 终章

恭喜你完成了"LifetimeKata". 在接下来的两章中还有更多的附注和额外阅读材料, 但这一章可以看作是一个"终章".

在这个练习中, 我们将构建一个非常简单的通配符系统的克隆版本. 这允许某人查询一段文本是否匹配某个描述.

值得注意的是, 实现整个功能可能需要最多一个小时. 如果你只想处理生命周期部分, 你可以从`solution`中复制代码, 但完整的完成这个练习是一个有趣而且有益.

例如, 通配符`ab(cd|ef|gh)`匹配以下任何字符串之一: `abcd`, `abef`, `abgh`.

您将创建一个`Matcher`结构体, 其中包括三个字段:

- 一个`&str`, 表示正则表达式的文本表示形式.
- 一个`Vec<MatcherTokens>`, 表示正则表达式中不同的部分, 按顺序排列.
- 一个整数, 用于跟踪正则表达式的最长匹配.

要创建这个, 你需要一个看起来像这样的字符串: `hello.(town|world|universe).its.me`.
这有三个组成部分:

- 普通文本, 比如 `'hello'`, `'its'` 或 `'me'`, 它们只匹配精确的文本.
- 通配符(`.`字符), 可以匹配任意单个字符.
- 可选文本, 比如 `(town|world|universe)`, 它可以匹配列表中的一个字符串, 所以 `(town|world|universe)` 可以匹配 `town`, 或者 `world`, 或者 `universe`.

这些可以以任何顺序混合和匹配(但永远不会有一个在另一个内部). 使用这个字符串, 你应该创建一个`MatcherTokens`的`vector`, 它们指向该字符串的相关部分.

然后, 您将编写一个函数, 该函数接受另一个字符串, 并查看`Matcher`与该特定字符串匹配了多少. 您将返回一个由`(MatcherToken, &str)`组成的`vector`, 其中`MatcherToken`是匹配了一些文本的标记, 而`&str`是已匹配的文本.

## 一个例子

假设你有匹配器`(Black|Bridge)(rock|stone|water).company`.这可以分为四个部分:

- `OneOfText(["Black", "Bridge"])`
- `OneOfText(["rock", "stone", "water"])`
- `Wildcard`
- `RawText("company")`

现在, 让我们假设我们有以下文本: `BlackBridge`. `Black`匹配了第一个标记, 但`Bridge`不匹配第二个标记. 所以, 我们将返回: `vec![(OneOfText(["Black", "Bridge"]), "Black")]`. 我们匹配的最多标记数是`1`.

以另一个例子来说, 考虑B`ridgestone_Tyres`. `Bridge`匹配第一个匹配器, `stone`匹配第二个匹配器, _匹配第三个匹配器, 但`Tyres`与`company`不匹配. 所以, 我们匹配的最多标记数是`3`. 我们将返回一个包含以下内容的`vector`:

- (`OneOfText(["Black", "Bridge"])`, `Bridge`)
- (`OneOfText(["rock", "stone", "water"])`, `"stone"`)
- (`Wildcard`, `"_"`)

### 关于Unicode的说明

`Rust`能够处理其字符串中的`Unicode`字符(如表情符号或日本字). 当然, 这会增加执行简单操作, 比如将字符串拆分成多个部分所需的复杂性, 因为有可能意外地将一个字符分成两半.

例子中的测试不使用`Unicode`, 但是如果你想要一个真正的`Rust`体验, 可以将测试更改为包含一个`Unicode`字符(在注释中提供了一个示例).
