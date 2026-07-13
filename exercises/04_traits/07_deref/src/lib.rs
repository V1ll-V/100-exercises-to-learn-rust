// TODO: 每当通过访问器方法返回 `title` 和 `description` 时，
//   它们应该被规范化——即去除前导和尾随的空白字符。
//   Rust 标准库中有一个方法可以帮助实现这一点，但你在 `String` 的文档中
//   找不到它。
//   你能找出它在哪里定义以及如何使用它吗？

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
