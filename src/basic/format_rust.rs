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

# 格式化占位符
## {}
`{} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，`
## {:?}
`{:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景`

## {:#?}
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

    println!("{}", ss);
    println!("{:?}", ss);
    println!("{:#?}", ss);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

/**
# {:?} 与 {:#?}
`{:#?} 更优美`
````
{:?}
Person { name: "san", age: 18 }

{:#?}
Person {
    name: "san",
    age: 18,
}
````
*/

pub fn format_struct() {
    let person = Person {
        name: String::from("san"),
        age: 18,
    };

    println!("{:?}", person);
    println!("{:#?}", person);
}
