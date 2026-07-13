use ticket_fields::{TicketDescription, TicketTitle};

// TODO: 让我们开始搭建 ticket store！
//  第一个任务：为 `TicketStore` 实现 `IntoIterator`，使其可以通过 `for` 循环
//  遍历其中包含的所有 ticket。
//
// 提示：这种情况下你不需要实现 `Iterator` trait。
//   你想要将迭代 *委托* 给 `TicketStore` 中的 `Vec<Ticket>` 字段。
//   查阅标准库文档中 `Vec` 的相关部分，找到 `into_iter` 应该返回
//   的正确类型。
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
