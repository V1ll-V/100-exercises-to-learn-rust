// TODO: 这是一个孤儿规则违规的示例。
//  我们在一个外部类型（`u32`，来自 `std`）上实现了一个外部 trait（`PartialEq`，来自 `std`）。
//  查看编译器错误以熟悉它的样子。
//  然后删除下面的代码，继续下一个练习。

impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
