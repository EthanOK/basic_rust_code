// rust 有理数与复数 需导入 num library
use num::complex::Complex;

use crate::basic::utils::type_of;
pub fn add_complex() {
    let a = Complex { re: 2.0, im: 5.0 };
    let b = Complex::new(3.0, 4.0);
    let c = a + b;
    println!("{}", c);
    println!("{}", type_of(&c));
}
