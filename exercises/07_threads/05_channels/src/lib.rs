use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// 启动系统，通过启动服务器线程。
// 它返回一个 `Sender` 实例，然后可以由一个或多个
// 客户端使用来与服务器交互。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 服务器任务应该**永不**停止。
//  进入一个循环：等待通道中出现命令，
//  然后执行它，然后开始等待下一条命令。
pub fn server(receiver: Receiver<Command>) {}
