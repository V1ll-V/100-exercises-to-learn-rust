// TODO: 为 `min` 添加必要的 trait 约束，使其能够成功编译。
//   参考 `std::cmp` 模块的文档，了解你可能需要的 trait。
//
// 注意：有多种 trait 约束可以让编译器满意，但它们具有不同的_语义_。
// 我们将在课程后面讨论有序集合（例如 BTreeMap）时讲解这些差异。

/// 返回两个值中的较小者。
pub fn min<T>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
