use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}
pub fn display_complex() {
    let complex = Complex {
        real: 2.1,
        imaginary: 7.5,
    };

    println!("Display: {}", complex);

    println!("Debug: {:?}", complex);
}
