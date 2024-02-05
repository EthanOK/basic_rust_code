/*
scalar type: integer float bool chart
*/

use std::mem::size_of_val;

pub fn def_int_type() {
    // 整数类型
    // i8 i16 i32 i64 i128 isize
    // u8 u16 u32 u64 u128 usize

    // isize 和 usize 取决于系统 CPU 多少位

    // 默认整数类型是 i32

    // 有符号 32 位整数
    let _x: i32 = -5;
    // 有符号 32 位整数
    let _y: u32 = 5;

    // 整数溢出
    // checked_add() 返回 None 则 发生 溢出
    let z = u8::MAX.checked_add(1);

    println!("{:?}", z);

    // wrapping_add 补码循环溢出  255 + 2 = 1
    let z = u8::MAX.wrapping_add(2);

    println!("{:?}", z);

    // overflowing_add  返回该值 和 是否溢出的布尔值 (4, true)
    let z = u8::MAX.overflowing_add(5);

    println!("{:?}", z);

    // saturating_add 计算后的值不超过 目标类型的 最大值 或 最小值
    // 255.saturating_add(6) => 255
    let z = u8::MAX.saturating_add(6);

    println!("{:?}", z);
}

pub fn def_float_type() {
    // 浮点数类型
    // f32 f64

    // 默认浮点数类型是 f64

    // f32 单精度 f64 双精度

    let _f1 = 2.1;
    let _f2: f32 = 2.1;

    // assert_eq!(0.1 + 0.2, 0.3);
    /*
    避免在浮点数上测试相等性
    不能作为HashMap的Key
    */

    println!("{}", (0.1_f64 + 0.2 - 0.3).abs() < 0.000001);

    let nan = (-42.1_f64).sqrt();
    if nan.is_nan() {
        println!("Result is NaN");
    }
}

pub fn numeric_operate() {
    // 只能同类型才能参与计算

    // 编译器 自动推导数据类型
    let sum = 5 + 10;

    println!("{}", sum);

    // 不同类型之间不能直接运算 23.9 - 5
    let difference = 95.5 - 4.0;
    println!("{}", difference);

    let product = 4 * 30;
    println!("{}", product);
    // 除法
    let quotient = 56 / 32;
    println!("{}", quotient);
}

pub fn bit_operate() {
    // 位运算
    // & | ^ ! << >>

    // 00000010
    let a = 2;
    // 00000011
    let b = 3;
    println!("(a & b) = {}", a & b);
    println!("(a | b) = {}", a | b);
    // 异或
    println!("(a ^ b) = {}", a ^ b);
    println!("(!b) = {}", !b);
    println!("(a << b) = {}", a << b);
    println!("(a >> b) = {}", a >> b);

    let mut a = a;
    // a = a << b;
    a <<= b;
    println!("(a << b) = {}", a);
}

pub fn bool_type() {
    // 布尔类型
    // bool

    let t = true;
    let f: bool = false;

    println!("{:?}", t);
    println!("{:?}", f);
}

pub fn char_type() {
    // 字符类型
    // char
    // 占用 4 个字节
    // Rust 中的字符包括 ASCII 和 Unicode
    // 字符只能用 '' 表示

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '🌝';

    println!("{:?}", c);
    println!("{:?}", z);
    println!("{:?}", heart_eyed_cat);

    println!("{:?}占用 {} 个字节", c, std::mem::size_of_val(&c))
}

pub fn unit_type() {
    // 单元类型 （） 不占用内存
    // main() 的返回值 就是 单元类型，并不是无返回值

    //
    let unit = ();

    assert!(unit == ret_unit_type());

    println!("{:?} 占用 {} 个字节", unit, size_of_val(&unit));
}

pub fn ret_unit_type() {
    println!("return ()");
}
