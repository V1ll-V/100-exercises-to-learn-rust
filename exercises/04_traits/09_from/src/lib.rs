// TODO: 为 `WrappingU32` 类型实现 `From` trait，使 `example` 能够编译。

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
