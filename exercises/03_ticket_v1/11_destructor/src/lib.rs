// 我们需要更多的基础设施来为析构函数编写一个合适的练习。
// 在后续章节中，我们会再次涉及 trait 和内部可变性之后
// 重新拾起这个概念。
fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
