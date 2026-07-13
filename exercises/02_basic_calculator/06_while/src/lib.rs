// 使用 `while` 循环重写阶乘函数。
pub fn factorial(mut n: u32) -> u32 {
    // `todo!()` 宏是一个占位符，编译器
    // 会将其解释为"我稍后再回来实现"，从而
    // 抑制类型错误。
    // 它在运行时会 panic。
    //todo!()
    while n == 0 {
        return 1;
    }
    let mut result = 1;
    while n > 0 {
        result *= n;
        n -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
