/*
Reference：指针类型，指向 对象存储的 内存地址
Borrowing: 获取变量的引用，称为 借用

引用 指向的值 默认是不可变的
 */

/*
&mut: 可变引用 同一个作用域，只能存在一个
可任意多个不可变引用

可变引用与不可变引用 不能 同时存在

⚠️ 引用的作用域：从创建开始，持续到最后使用的地方

 */

pub fn ref_bor() {
    let a = 10;

    // b 是 a 的一个引用，它指向 a
    let b = &a;
    println!("a = {}, *b = {:p}", a, b);
    assert_eq!(a, *b);
}

pub fn read_string_len() {
    let s1 = String::from("hi");

    println!("{:?} length: {}", s1, caculate_len(&s1));
}

pub fn change_string_data() {
    let mut s1 = String::from("hi");
    change_string(&mut s1);
    println!("{:?} length: {}", s1, caculate_len(&s1));
}

fn caculate_len(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}

pub fn mut_reference() {
    let mut s = String::from("hello");

    let s1 = &mut s;
    println!("{}", s1);
    let s2 = &mut s;
    println!("{}", s2);
}
