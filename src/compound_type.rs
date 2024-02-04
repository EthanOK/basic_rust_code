/*
字符串(UTF8编码) 结构体 枚举 数组 元组 切片(是对集合的部分引用)
 */

// #![allow(unused_variables)]
type File = String;
#[allow(dead_code)]
const MAX: u8 = u8::MAX;

pub fn remove_warning() {
    #[allow(unused_variables)]
    let x = 10;

    let mut file_1 = File::from("src/1.txt");

    open(&mut file_1);
    // read(&mut file_1, &mut vec![]);
    close(&mut file_1);
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
    // unimplemented!() 告诉编译器函数尚未实现，执行到，程序会报错
}

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

pub fn read_slice() {
    // 切片 右半开区间
    let s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[6..11];
    println!("{}, {}", s1, s2);

    // 从 索引 0 开始 ， 2 种方式
    let slice_start = &s[0..2];
    let slice_start = &s[..2];

    // 包含最后一个字节 ， 2 种方式
    let len = s.len();

    let slice_end = &s[6..len];
    let slice_end = &s[6..];

    // 截取完整 String 切片
    let slice_whole = &s[0..len];
    let slice_whole = &s[..];

    // 切片的索引必须落在字符组之间的边界位置
    // 字符串字面量 是 切片
    let st = "打鬼子";
    println!("{}", st.len());
    // error: byte index 2 is not a char boundary; it is inside '打' (bytes 0..3) of `打鬼子`
    // let st_1 = &st[0..2];
    let st_1 = &st[0..3];
    println!("{}", st_1);
}

pub fn str_convert_string() {
    let s = "hello";
    let string_ = String::from(s);
    println!("{}", string_);

    let string_ = s.to_string();

    println!("{}", string_);
}

pub fn string_convert_str() {
    let s = String::from("hello");
    let s_1 = &s[..];
    let s_2 = s.as_str();
    let s_3 = &s;
    println!("{}, {}, {}", s_1, s_2, s_3);
}

pub fn string_index() {
    let s = String::from("hello");
    // 字符串的底层数据格式是 [u8], 一个字节数组
    // error
    // let h = s[0];

    // 不允许去索引字符串

    println!("{}", s);
}
