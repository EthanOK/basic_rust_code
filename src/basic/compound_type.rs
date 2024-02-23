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

/*
元组: 长度固定，顺序固定
    场景：返回值
 */
pub fn tuple_type() {
    // 创建元组
    let t = (1, "hello");

    // 解构元组
    let (a, b) = t;
    println!("{}, {}", a, b);

    // 访问元组
    println!("({}, {})", t.0, t.1)
}

/*
枚举 enum
任何类型的都数据都可以放入枚举成员
Option<T>枚举，有值：Some(T) 无值：None ，T是一个泛型参数
 */

#[derive(Debug)]
enum TokenType {
    NULL,
    ERC20,
    ERC721,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum_type() {
    let token_type = TokenType::NULL;
    println!("{:?}", token_type);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(1, 2, 3);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    println!("{:?}", m4);

    // 使用 None 需要告诉  Option<T> 是什么类型
    let v1: Option<i32> = None;

    let v2 = Some(888);

    let v3 = Some("a string");
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);

    println!("{}", v1.unwrap_or_default());
    println!("{}", v2.unwrap_or_default());
}

/*
长度固定： array 元素类型相同 顺序存放
动态数组： Vector
 */
pub fn array_type() {
    // 长度固定 存储在栈上 声明: [type; num]
    let arr: [i8; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", arr);

    // 初始化 某个值 重复 N 次的数组  =[value; count]
    let aar0 = [0; 4];
    println!("{:?}", aar0);

    println!("{}", arr[0]);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    id: u32,
}
pub fn struct_type() {
    // 每个字段都要初始化 顺序可以不一致
    let user1 = User {
        username: String::from("zhang san"),
        id: 1,
        email: String::from("888888@gmail.com"),
    };
    println!("{:?}", user1);

    let user2 = build_user("DJ".to_string(), 2);
    println!("{:?}", user2);

    // 结构体更新 语法 ..user1 必须在尾部
    let user3 = User {
        username: String::from("wang wu"),
        ..user1
    };
    println!("{:?}", user3);
    // username 和 email 的所有权 转移给了 user3
    // println!("{:?}", user1.email);
}

fn build_user(username: String, id: u32) -> User {
    User {
        username,
        email: String::from(""),
        id,
    }
}

pub fn tuple_struct() {
    // 元组结构体 不需要初始化，可以直接使用
    #[derive(Debug)]
    struct Pair(i32, i32);

    let pair = Pair(1, 2);
    println!("{:?}", pair);

    // 元组结构体 不需要初始化，可以直接使用
    #[derive(Debug)]
    struct Point3D(i32, i32, i32);

    let point = Point3D(1, 2, 3);

    // 拿走表达式所有权，还会把值的所有权返回
    dbg!(&point);

    println!("{:?}", point);
}
