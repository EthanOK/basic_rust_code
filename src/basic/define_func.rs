// 由 表达式 来返回值
// 表达式 不能包含 ';'

// 函数名和变量名 使用 蛇形命名法

pub fn add(a: i32, b: i32) -> i32 {
    let a = a + 1; // 语句
    let b = b - 1; // 语句
    a + b // 表达式
}

pub fn statement_expression() {
    let x = 5;

    let y = {
        let x = x * x;
        let y = x + x;
        x + y
    };

    let z = {
        let x = x * x;
        let y = x + x;

        // x + y; 等价于  let _ = x + y;
        x + y;
    };

    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);
}
