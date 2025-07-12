# rust-by-practice
[Rust 语言圣经](https://course.rs/about-book.html)的配套习题练习记录。

来源: [Rust By Pracrice](https://practice-zh.course.rs/why-exercise.html)

对答案：[Solution](https://github.com/sunface/rust-by-practice/tree/master/solutions)

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

## 我的错题本
[5-2-4](src/bin/5-2-4.rs)

