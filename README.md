# rust-by-practice
[Rust 语言圣经](https://course.rs/about-book.html)的配套习题练习记录。

来源: [Rust By Pracrice](https://practice-zh.course.rs/why-exercise.html)

对答案：[Solution](https://github.com/sunface/rust-by-practice/tree/master/solutions)

这套习题前面都还比较简单，从第 10 章开始难度就逐渐上来了。到后面还有一大堆 todo，作者太忙不更了。不过做完现有部分也差不多了。注意有一部分题是社区贡献的，只有英文版没有中文版。

## 编译参考:
- 使用`cargo`：
```bash
cargo run --bin 3-1
```
此法要求文件全都放在bin路径下，但我现在在用 [zed](https://github.com/zed-industries/zed)，没找到更好的方案。
- 使用`VS Code`的`code runner`插件：

为避免生成的二进制文件污染目录，需修改插件配置:

**前**
```json
"code-runner.executorMap": {
    "rust": "cd $dir && rustc $fileName && $dir$fileNameWithoutExt",
}
```
**后**
```json
"code-runner.executorMap": {
    "rust": "cd $dir && rustc $fileName && $dir$fileNameWithoutExt && rm $dir$fileNameWithoutExt",
}
```
然后`Ctrl+Alt+N`或点击右上角三角形直接运行。

## 批量创建文件
```bash
touch 3-1-{1..9}.rs
```

## 文件内容
题目说明与星级

题目原文

我的解答

参考答案（如和我的解答有不同）

## 我的错题本
[5-2-4](src/bin/5-2-4.rs)

[7-10](src/bin/7-10.rs)

[8-1-4](src/bin/8-1-4.rs)

[8-2-6](src/bin/8-2-6.rs)

[10-1-2](src/bin/10-1-2.rs)

[10-1-6](src/bin/10-1-6.rs)

[10-2-3](src/bin/10-2-3.rs) 需要 nightly 版本才能运行

[10-4-5](src/bin/10-4-5.rs)

[10-5-1](src/bin/10-5-1.rs) [10-5-2](src/bin/10-5-2.rs)

[11-3-2](src/bin/11-3-2.rs)

[12-1-3](src/bin/12-1-3.rs)

[12-3-2](src/bin/12-4-3.rs)