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

pub fn matches_demo() {
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = [MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    let mut count = 0;

    let pattern = MyEnum::Foo;

    for e in v {
        // matches!(expr, pattern) pattern 参数必须是模式 而不能是变量
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    println!("{:?}出现了 {} 次", pattern, count);
}

pub fn match_mut_ref() {
    let mut s = String::from("hello");

    let v = &mut s;
    // v be move. because `v` has type `&mut String`,
    // which does not implement the `Copy` trait
    match v {
        value => println!("{:?}", value),
    }

    // println!("{:?}", v);

    println!("{}", s);
}
