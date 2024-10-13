pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

// 定义一个新的 trait，继承 `SomeTrait` 和 `OtherTrait`
pub trait CombinedTrait: SomeTrait + OtherTrait {}

impl CombinedTrait for SomeStruct {}
impl CombinedTrait for OtherStruct {}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 修改函数参数类型为 `CombinedTrait` 的 trait 对象
fn some_func(item: &dyn CombinedTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(&SomeStruct {});
    some_func(&OtherStruct {});
}
