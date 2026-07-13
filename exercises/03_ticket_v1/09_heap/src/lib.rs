pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为
//  相应类型的正确**栈大小**。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
        // 这是一个有难度的问题！
        // "直觉性"的答案这次恰好也是正确答案，
        // 但一般来说，结构体的内存布局是一个更复杂的话题。
        // 如果你感兴趣，可以查阅 Rust 参考手册的"类型布局"章节
        // https://doc.rust-lang.org/reference/type-layout.html 了解更多信息。
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
