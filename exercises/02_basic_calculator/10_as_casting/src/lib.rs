// TODO: 根据你在本节学到的知识，将 `todo!()` 替换为
//  转换后的正确值。

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47u16 as u32;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // 编译器足够聪明，知道值 255 无法放入
        // i8，因此会发出硬错误。我们故意禁用
        // 这个护栏，使得这个（不好的）转换成为可能。
        // 编译器之所以能检测到这个问题，是因为该值是一个
        // 字面量。如果我们使用变量，编译器将无法
        // 在编译时捕获此问题。
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // 你可以使用与上面完全相同的表达式来解决这个问题，
        // 但那样就失去了本练习的意义。请改用一个真正的
        // `i8` 值，当转换为 `u8` 时等价于 `255`。
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = true as u8;
        assert_eq!(true as u8, v);
    }
}
