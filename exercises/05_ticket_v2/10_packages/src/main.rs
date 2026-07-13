// 这是一个 `main.rs` 文件，因此 `cargo` 将其解释为二进制目标的根。

// TODO: 修复这个损坏的导入。在 `src` 目录中创建一个新的库目标。
//   库目标应暴露一个名为 `hello_world` 的公共函数，该函数不接受参数且不返回任何值。
use packages::hello_world;

// 这是二进制的入口点。
fn main() {
    hello_world();
}
