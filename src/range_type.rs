pub fn range_number() {
    // 序列 Range
    // 1..10 代表 连续数值 1,2,3,4,..,9

    for i in 1..10 {
        print!("{},", i);
    }

    println!();

    // 1..=10 代表 连续数值 1,2,3,4,..,10
    for i in 1..=10 {
        print!("{},", i);
    }

    println!();
}

pub fn range_char() {
    // 序列 Range
    // 'a'..='z' 代表 连续字符 a,b,c,..,y,z

    for i in 'a'..='z' {
        print!("{}, ", i);
    }

    println!();
}
