use std::{fs::File, io::ErrorKind};

/**
 * Rust Error: 可恢复错误 Result<T,E> ，不可恢复错误 panic!
 *
 */

pub fn panic_demo() {
    let v = vec![1, 2, 3];

    // 被动触发：数组越界 panic
    v[99];

    // 主动调用
    if true {
        panic!("panic error");
    }
}

pub fn result_demo() {
    let f = File::open("util.txt");
    let r = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("util.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("create error{e:?}"),
            },
            _ => panic!("error"),
        },
    };
}

// 失败就 panic 的函数：unwrap() expect()
pub fn panic_unwrap_demo() {
    let f = File::open("util.txt").unwrap();
}

pub fn panic_expect_demo() {
    let f = File::open("util.txt").expect("open error");
}

/**
 * ? 操作符：传播
 * 需要一个变量来承载正确的值
 */

pub fn wenhao_demo() {
    let arr = [];

    let r = get_first(&arr);
    println!("{:?}", r);

    let ss = "ojbk";
    let r = get_last_of_first_line(ss);
    println!("{:?}", r);
}

fn get_first(arr: &[i32]) -> Option<&i32> {
    // 如果 arr.get(0) 返回None，直接返回 None
    let r = arr.get(0)?;
    Some(r)
}

fn get_last_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
