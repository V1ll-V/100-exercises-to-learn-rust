// TODO: 实现一个所谓的"Drop 炸弹"：一种在丢弃时会 panic 的类型，
//  除非对其执行了某个特定操作。
//  你可以在下面的测试中看到预期的 API。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // 炸弹在丢弃时应该 panic
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // 炸弹在丢弃时不应该 panic
        // 因为它已经被拆除了
    }
}
