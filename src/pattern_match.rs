#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

pub fn match_demo() {
    let dire = Direction::South;

    // 穷尽匹配
    match dire {
        Direction::East => println!("East"),

        Direction::North | Direction::South => println!("North or South"),
        // _通配符 放置其他分支后
        _ => println!("West"),
    }

    match dire {
        Direction::East => println!("East"),

        Direction::North => println!("North"),
        // 用一个变量 承载其他情况
        other => println!("{:?}", other),
    }
}

pub fn if_let_demo() {
    let v = Some(8);

    // 只有一个模式需要被处理
    if let Some(i) = v {
        println!("{}", i);
    } else {
        println!("None");
    }
}
