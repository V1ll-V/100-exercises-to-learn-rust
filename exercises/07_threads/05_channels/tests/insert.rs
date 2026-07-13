// TODO: 当你认为自己完成了这个练习时，在 `ready` 中将 `move_forward` 设置为 `true`。
//  随时可以叫一位导师来验证你的解决方案！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // 如果线程不再运行，这将会 panic，
        // 因为通道已经关闭。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // 在这个练习中，我们可以自动检查的内容非常有限，
    // 因为我们的服务器没有暴露任何**读取**操作。
    // 我们无法知道插入操作是否真的发生了，以及是否
    // 正确发生。
    let move_forward = false;

    assert!(move_forward);
}
