pub fn u16_as_i32() {
    let a: u16 = 100;

    let b: i32 = -200;

    let c = a as i32 + b;
    println!("{}", c);
}
