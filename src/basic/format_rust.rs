/*!
# 格式化输出
````
print! 将格式化文本输出到标准输出，不带换行符

println! 同上，但是在行的末尾添加换行符

format! 将格式化文本输出到 String 字符串
````
## print!
## println!
## format!
*/

/**
# format!
`格式化文本输出到 String 字符串，返回一个 String 字符串`

```
let s = "hello";
let s:String = format!("{} world", s);
assert_eq!(s, "hello world");
```
*/

pub fn format_to_string() {
    let s = "hello";

    let ss = format!("{} world", s);

    println!("{ss}");
}
