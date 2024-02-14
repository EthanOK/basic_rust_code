/**
 * 泛型 <T>
 */

pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 数组切片 声明 &[T]
pub fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 结构体中使用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
pub fn struct_generics() {
    let p0 = Point { x: 1, y: 2 };
    let p1 = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p0);
    println!("{:?}", p1);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 枚举中使用泛型
pub fn enum_generics() {
    let option = Option::Some(2);
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
pub fn func_generics() {
    let p0 = Point::new(3.14, 4.55);

    println!("p0.x = {}", p0.x());
}
