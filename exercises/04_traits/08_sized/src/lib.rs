pub fn example() {
    // 尝试通过 `std::mem::size_of` 获取 str（或任何其他 DST）
    // 的大小会导致编译时错误。
    //
    // TODO: 注释掉下面这行，然后继续下一个练习。
    std::mem::size_of::<str>();
}
