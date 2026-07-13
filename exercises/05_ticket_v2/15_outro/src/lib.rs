// TODO: 在这个 crate 的每个模块中都有一些事情要做！
mod description;
mod status;
mod title;

// Rust 中一种常见的模式是将代码拆分为多个（私有）模块，
// 然后在 crate 的根级别重新导出这些模块的公共部分。
//
// 这样可以对用户隐藏 crate 的内部结构，同时
// 仍然允许你按照自己喜欢的方式组织代码。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// 我们不再需要将字段设为私有了！
// 由于每个字段都封装了自己的验证逻辑，不存在
// `Ticket` 的用户修改字段从而破坏结构体不变量的风险。
//
// 但要注意：如果你有任何跨越多个字段的不变量，你
// 需要确保这些不变量仍然得到维护，并重新将字段设为私有。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
