pub fn if_control() {
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("{}", num);

    let n = 6;
    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3, or 2");
    }
}

pub fn for_control() {
    let arr = [String::from("value"), "ee".to_string()];

    // 要加 &，不然 所有权 会被移动到 for 循环中， 后续不能再访问
    // 实现 Copy 特征的数组 [i32; 12], 不需要加&
    for a in &arr {
        println!("{}", a);
    }

    println!("{:?}", arr);

    let a = [1, 2, 3, 4, 5];
    for (index, value) in a.iter().enumerate() {
        println!("the value at {} is {}", index, value);
    }
    let mut sum = 100;
    for _ in 0..10 {
        sum -= 5;
    }
    println!("{}", sum);

    // continue 跳过当前当次的循环
    for i in 1..4 {
        if i == 2 {
            continue;
        } else {
            println!("{}", i);
        }
    }

    // break 跳出当前整个循环
    for i in 1..4 {
        if i == 2 {
            break;
        } else {
            println!("{}", i);
        }
    }

    let mut n = 0;
    while n < 5 {
        println!("{}!", n);
        n += 1;
    }
    println!("out while");

    // loop 无条件循环 if break

    loop {
        if n == 1 {
            break;
        }

        println!("{} in loop", n);
        n -= 1;
    }

    println!("out loop");
}
