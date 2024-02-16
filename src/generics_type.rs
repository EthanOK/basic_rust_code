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

// 为具体的泛型类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn func_generics() {
    let p0 = Point::new(3.14, 4.55);

    println!("p0.x = {}", p0.x());

    let p1 = Point::new(4.0_f32, 3.0_f32);

    println!("p1.distance_from_origin = {}", p1.distance_from_origin());
}

// const 泛型表达式

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
    println!("size_of = {}", core::mem::size_of::<[T; N]>());
}

pub fn print_array() {
    let arr = [1, 2, 3, 4, 5];

    display_array(arr);
    let arr = [1.0, 2.0, 3.0];
    display_array(arr);
}
