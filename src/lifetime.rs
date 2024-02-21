/**
 * 引用 &i32
 * &'a i32
 * &'a mut i32
 *
 * 函数的返回值如果是一个引用类型,那么他的生命周期只来源于：
 * 1. 函数参数的生命周期
 * 2. 函数体中某个新建引用的生命周期
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
