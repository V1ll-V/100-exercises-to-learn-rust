//! TODO: 通过**重新排序** `example` 函数中的语句来让代码编译通过。
//!  不允许修改 `spawner` 函数，也不允许修改 `example` 中每行代码的作用。
//!  如果需要的话，你可以将现有语句包裹在块 `{}` 中。
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
