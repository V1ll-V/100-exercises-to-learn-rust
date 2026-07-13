// 给定一个数字 `n`，返回斐波那契数列中的第 `n+1` 个数。
//
// 斐波那契数列定义如下：
//
// - 数列的第一个数是 0。
// - 数列的第二个数是 1。
// - 每个后续的数都是前两个数之和。
//
// 因此数列为：0, 1, 1, 2, 3, 5, 8, 13, 21，依此类推。
//
// 我们期望 `fibonacci(0)` 返回 `0`，`fibonacci(1)` 返回 `1`，
// `fibonacci(2)` 返回 `1`，依此类推。
pub fn fibonacci(n: u32) -> u32 {
    // TODO: 实现 `fibonacci` 函数
    //
    // 提示：使用 `Vec` 来记忆已计算的结果，
    // 这样就不需要多次重复计算。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
