// TODO: 使用 `spawn` 和 `join` 实现 `sum` 函数的多线程版本。
//  给定一个整数向量，将其分成两半，并在两个独立的线程中各计算一半的和。

// 注意：我们无法测试函数是*如何*实现的，
// 只能验证它产生了正确的结果。
// 你_可以_仅通过返回 `v.iter().sum()` 来通过这个测试，
// 但那会失去这个练习的意义。
//
// 提示：你不能让已启动的线程直接_借用_向量的切片。
// 你需要为原始向量的每一半分配新的向量。我们将在下一个练习中
// 看到为什么这是必要的。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
