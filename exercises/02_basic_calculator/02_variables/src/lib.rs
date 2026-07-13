// 👇 下面以 `///` 开头的行称为**文档注释**。
//    它们为紧随其后的项附加文档。在这里是为 `speed` 函数附加文档。
//    如果你在此练习目录下运行 `cargo doc --open`，Rust 会从这些注释
//    生成 HTML 文档并在浏览器中打开。

/// 给定一段旅程的起点和终点，以及完成该旅程所花的时间，
/// 计算平均速度。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 定义一个名为 `distance` 的变量并赋予正确的值，使测试通过
    //  你需要标注 `distance` 的类型吗？为什么需要或为什么不需要？

    // 不要修改下面这行
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
