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
}
