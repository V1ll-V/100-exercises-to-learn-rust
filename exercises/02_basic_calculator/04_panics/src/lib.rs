/// 给定一段旅程的起点和终点，以及完成该旅程所花的时间，
/// 计算旅程的平均速度。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 如果 `time_elapsed` 为 0，则用自定义消息触发 panic
if time_elapsed == 0 {
    panic!("The journey took no time at all. That's impossible!");
}
    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 通过 `#[should_panic]` 注解，我们可以断言被测代码
    //    应该 panic。我们也可以通过 `expected` 来检查 panic 消息。
    //    这都是 Rust 内置测试框架的一部分！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
