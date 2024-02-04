/*
push: 在原有字符串上追加，并不会返回新字符串
    1.push_str 追加字符串
    2.push 追加 char

insert: 在原有字符串中插入，并不会返回新字符串,索引从0开始，越界则报错
    1.insert_str 插入字符串
    2.insert 插入 char

replace: 替换字符串
    1. 使用 replace_range(index, ""), 仅适用于 String, 直接操作原来的字符串，不返回
    2. 使用 replace(old, new), 返回新的字符串(String),适用于String和&str
    3. 使用 replacen(old, new, count), count 表示替换的数量. 返回新的字符串(String),适用于String和&str
 */

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
