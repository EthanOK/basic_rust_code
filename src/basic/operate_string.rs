/*!
# 字符串操作

## push: 在原有字符串上追加，并不会返回新字符串
    1.push_str 追加字符串
    2.push 追加 char

## insert: 在原有字符串中插入，并不会返回新字符串,索引从0开始，越界则报错
    1.insert_str 插入字符串
    2.insert 插入 char

## replace: 替换字符串
    1. 使用 replace_range(index, ""), 仅适用于 String, 直接操作原来的字符串，不返回
    2. 使用 replace(old, new), 返回新的字符串(String),适用于String和&str
    3. 使用 replacen(old, new, count), count 表示替换的数量. 返回新的字符串(String),适用于String和&str

 ## delete: 删除字符串
    1. 使用 pop(), 删除最后一个 char，并返回 char。直接操作原来的字符串
    2. 使用 remove(index), 删除并返回字符串中指定位置的char，直接操作原来的字符串
    3. 使用 truncate(index), 删除字符串中指定位置至结尾的全部字符串, 无返回值，直接操作原来的字符串
    4. 使用 clear(), 清空字符串,，直接操作原来的字符串，= truncate(0)

## concatenate: 连接字符串
    1. 使用 +, 返回新的字符串, String + &str
    2. 使用 format!(), 返回新的字符串, 适用于 String 和 &str

*/

use std::mem::size_of_val;

pub fn push_string() {
    let mut s = String::from("hello");
    // 1
    s.push_str(", world!");
    println!("{}", s);
    // 2
    s.push('好');
    println!("{}", s);
}

pub fn insert_string() {
    let mut s = String::from("hello");
    // 1
    s.insert_str(0, "world");
    println!("{}", s);
    // 2
    s.insert(s.len(), '!');
    println!("{}", s);
}

pub fn replace_string() {
    let mut s = String::from("hello world");
    // 1
    s.replace_range(6..11, "rust");
    println!("{}", s);

    // 2
    let ss = String::from("I like rust.");

    let ss_new = ss.replace("rust", "Rust");
    println!("{}", ss_new);
    let ss_str = "I like rust.";
    let ss_new = ss_str.replace("like", "don't like");
    println!("{}", ss_new);

    // 3
    let sss = "I like rust. But bob don't like rust!";

    let sss_new = sss.replacen("rust", "Rust", 1);
    println!("{}", sss_new);
}

pub fn delete_string() {
    let mut string_pop = String::from("hello 龙年大吉！");

    let p1 = string_pop.pop();
    println!("{:?}", p1);

    println!("{}", string_pop);

    let mut string_remove = String::from("大吉，大利");
    println!(
        "{:?} 占用 {} 个字节",
        string_remove,
        size_of_val(string_remove.as_str())
    );
    let remove = string_remove.remove(6);
    println!("{}", remove);
    println!("{}", string_remove);

    let mut string_truncate = String::from("今晚吃鸡!哈哈");
    string_truncate.truncate(12);
    println!("{}", string_truncate);

    let mut string_clear = String::from("牛气冲天");
    string_clear.clear();
    println!(
        "{:?} 占用 {} 个字节",
        string_clear,
        size_of_val(string_clear.as_str())
    );
}

pub fn concatenate_string() {
    let s1 = String::from("hello ");
    let s2 = String::from("world");

    // Must: String + &str
    // + 的 右边 必须是 字符串的切片引用 &str,
    // std::string::add(self, s: &str) -> String
    // &String 可以隐式转换 &str
    //  s1 所有权 转移至 add(), 调用后释放，再使用会报错
    let s3 = s1 + s2.as_str();
    // println!("{}", s1);

    let s4 = s3 + " " + &s2;
    println!("{}", s4);

    let ss1 = String::from("leran");
    let ss2 = String::from("rust");

    let ss3 = format!("{} {}", ss1, ss2);

    println!("{}", ss3);
}

pub fn escape_string() {
    //使用 \ 忽略换行符
    let s = "hello world
    你好，少年！
    世界，\
    你好！
    ";
    println!("{}", s);

    // 若不需要转义 \\x52
    println!("{}", "ni hao \x52");
    println!("{}", r"haha \ndd");

    // 如果字符串中有双引号，在开头和结尾加 #，若还有歧义，继续加

    println!("{}", r#"haha: "san""#);
}

pub fn operate_utf8() {
    let s = String::from("中国人");
    // 以 Unicode 字符方式遍历
    for c in s.chars() {
        println!("{}", c);
    }

    // 以 字节 遍历

    for b in s.bytes() {
        println!("{}", b);
    }
}
