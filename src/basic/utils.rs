pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn size_of<T>(_: &T) -> usize {
    // caculate bytes size
    std::mem::size_of::<T>()
}

//  Vec<T> 转换为 [T; N]
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
