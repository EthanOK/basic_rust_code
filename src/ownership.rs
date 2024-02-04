/*
内存安全：所有权
垃圾回收： Java Go
 */

/*
栈：后进先出 数据大小已知
堆：在堆上存放数据，需请求一定大小内存空间（在堆上找空位，标记已使用，分配位置，返回该位置地址的‘指针’）
 */

/*
所有权原则：
1. Rust中每一个值都被一个变量所拥有，该变量称为值的 所有者
2. 一个值只能被一个变量所拥有，或说， 一个值只能拥有一个所有者
3. 当所有者（变量）离开作用域时，该值会被释放（drop）
 */

pub fn ownership_ins() {
    // 字符串字面值 被硬编码 不可变
    let s = "hello";

    println!("{}", s);

    let mut ss = String::from("hello");
    ss.push_str(" world");
    println!("{}", ss);
}

/*
基本数据类型 固定大小，存储在 ‘栈’ 上， 赋值 ‘自动拷贝’
String 类型 存储在 ‘堆’ 上，转移 所有权
 */
pub fn transfer_ownership() {
    let s = String::from("hello");
    // 将所有权转移给了 s1， 变量 s 被remove 了
    let s1 = s;
    // println!("{}", s); // s 不可用了
    println!("{}", s1);
}
