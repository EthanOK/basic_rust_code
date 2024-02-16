pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn size_of<T>(_: &T) -> usize {
    // caculate bytes size
    std::mem::size_of::<T>()
}
