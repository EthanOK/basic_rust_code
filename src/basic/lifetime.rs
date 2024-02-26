/*!
# 生命周期

* 引用 &i32

* &'a i32

* &'a mut i32
*
* 函数的返回值如果是一个引用类型,那么他的生命周期只来源于：
* 1. 函数参数的生命周期
* 2. 函数体中某个新建引用的生命周期
*
* 'a: 'b 表示 'a 比 'b 活得久
*
* 在结构体中只要有 &str,&i32 等引用类型, 都需要显示标记 生命周期
*/

pub fn learn_lifetime() {
    // let x;

    // {
    //     let y: i32 = 20;
    //     x = &y;
    //     // borrowed value does not live long enough
    // }
    // // y 作用域结束，被释放 x是一个 悬垂指针
    // println!("x = {}", x);
}

pub fn function_lifetime() {
    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let s3 = longest(s1.as_str(), s2.as_str());
    println!("s3 = {}", s3);
}

// <'a> 标注生命周期 告诉编译器  x、y和返回值至少活得和 ‘a 一样久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: i32,
}

pub fn struct_lifetime() {
    let student;

    let name = String::from("alice");
    student = Student {
        name: name.as_str(),
        age: 20,
    };

    println!("student = {:?}", student);
}

pub fn static_lifetime() {
    let s = "hello";
    let v = {
        let s1 = "world!";
        (s1.as_ptr() as usize, s1.len())

        // 变量 s1 的生命周期结束了；对于字符串字面量，直接被打包到二进制文件中，永远不会被 drop
    };
    let s2 = get_str_at_location(v.0, v.1);
    println!("s2 = {}", s2);
}
fn get_str_at_location(pointer: usize, len: usize) -> &'static str {
    unsafe {
        let slice = std::slice::from_raw_parts(pointer as *const u8, len);
        std::str::from_utf8_unchecked(slice)
        // 得到的 字符串字面量 的生命周期需要显示标明 'static
    }
}
