/*!
# my Rust utils
*/

/**
# 变量占的数据类型
```
use rust_course::basic::utils;
let a = "3";
let name = utils::type_of(&a);

```
*/
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

/**
# 变量占用字节大小

```
use rust_course::basic::utils;
let a = 36;
let size = utils::size_of(&a);
assert_eq!(size, 4);
```
*/
pub fn size_of<T>(_: &T) -> usize {
    // caculate bytes size
    std::mem::size_of::<T>()
}

/**
# Vec<T> 转换为 [T; N]

`动态数组转静态数据`

*/
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
