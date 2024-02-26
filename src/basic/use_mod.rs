/*!
# 使用 `use` 导入模块
`use rand::Rng;`
*/
use rand::{thread_rng, Rng};

/** `get_random` 获得[start, end]范围的随机数

```
use rust_course::basic::use_mod;
let random = use_mod::get_random(100,200);
```
*/
pub fn get_random(start: i32, end: i32) -> i32 {
    let rng = thread_rng().gen_range(start..=end);

    println!("random: {}", rng);
    rng
}
