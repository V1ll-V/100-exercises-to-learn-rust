// TODO: 修复下面的函数签名，使测试通过。
//  务必阅读编译器的错误信息——Rust 编译器是你在这门课程中的结对编程伙伴，
//  它通常会引导你走向正确的方向！
//
// 输入参数应该与返回类型具有相同的类型。
fn compute(a: u32, b: u32) -> u32 {
    // 不要修改函数体。
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}