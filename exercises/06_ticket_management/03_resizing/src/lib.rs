#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // 已达到最大容量
        assert_eq!(v.capacity(), 2);

        v.push(3); // 超出容量，需要重新分配

        // 你能猜到新的容量会是多少吗？
        // 注意，标准库对重新分配 Vec 的算法不做任何保证，
        // 因此未来可能会发生变化。
        assert_eq!(v.capacity(), todo!());
    }
}
