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
