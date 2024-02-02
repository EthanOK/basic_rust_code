pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn size_of_val<T>(v: &T) -> usize {
    std::mem::size_of_val(v)
}
