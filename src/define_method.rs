/*
 * method: impl 结构体 枚举 特征(Trait)
 *
 * &self 是 self:&Self 的缩写（Self 被实现方法的结构体类型，self 此类型的实例）
 *
 * self 有所有权 概念   &self 只 读取 ； &mut self 可以修改
 *
 * 方法名 与 结构体字段名 可相同,
 *
 * 关联函数：在 impl 中 且 没有 self 的函数；new 需使用 '::' 调用
 *
 * 允许为一个 struct 定义多个 impl
 */
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

pub fn caculate_area_struct() {
    let rect: Rectangle = Rectangle::new(20, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("The width: {}", rect.width);
    println!("The width: {}", rect.width());
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

pub fn call_enum() {
    let message = Message::Write("hhhh".to_string());

    message.call();
}
