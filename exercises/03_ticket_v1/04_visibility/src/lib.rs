mod ticket {
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **例外地**，本练习中你需要同时修改 `ticket` 模块和 `tests` 模块。
#[cfg(test)]
mod tests {
    // TODO: 在父模块中添加必要的 `pub` 修饰符，以消除下面 use 语句的编译器错误。
    use super::ticket::Ticket;

    // 但要小心！在你为了编译 use 语句而更改可见性之后，
    // 我们不希望下面这个函数编译通过！
    // 一旦你验证它确实无法编译，就把它注释掉。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // 尝试运行本练习时，你应该会看到这个错误：
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: 一旦你验证下面的代码无法编译，
        //   就注释掉这一行，继续下一个练习！
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // 这同样应该是不可能的，会抛出与上面类似的错误。
        // （只有在你注释掉上一个测试中有问题的行之后，
        // 才会在下一个编译阶段抛出编译错误！）
        //
        // 这证明了 `Ticket::new` 现在是获取 `Ticket` 实例的唯一途径。
        // 不可能创建一个带有非法标题或描述的 ticket！
        //
        // TODO: 一旦你验证下面的代码无法编译，
        //   就注释掉这些行，继续下一个练习！
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
