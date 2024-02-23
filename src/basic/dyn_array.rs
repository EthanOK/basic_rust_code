use crate::basic::utils::size_of;

/**
 * 动态数组 Vector
 *
 * Vec<T> 是一个泛型类型, T 是类型参数，表示该类型可以存储任意类型的值。
 */

pub fn create_vec() {
    // a 被显式声明
    let a: Vec<i32> = Vec::new();

    // aa 在添加元素时，推导出了具体类型
    let mut aa = Vec::new();
    aa.push(2);

    // 若预先知道存储的元素个数, 可以使用 with_capacity 方法创建动态数组。
    let mut aaa: Vec<i32> = Vec::with_capacity(10);

    aaa.push(22);
    println!("{:?}", aaa.len());

    // vec! 宏可以创建动态数组，并且推导出具体类型。

    let mut aaaa = vec![1, 2, 3];
    aaaa.push(4);
    println!("{:?}", aaaa);

    let a = vec![0; 4];
    println!("{:?}", a);
}

pub fn read_element_vec() {
    /***
     *1. 通过下标获取元素(用引用 &v[i])
     *2. 通过 get 方法获取元素
     *
     * 比较 索引越界
     * 确定索引不会越界用 下标 获取元素，不确定用 get 方法获取元素
     */
    let v = vec![1, 2, 3, 4, 5];

    let value = &v[2];
    println!("{}", value);

    match v.get(2) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}

pub fn iter_vec() {
    let v = vec![1, 2, 3, 4, 5];

    // 比用 下标的方式 更安全高效
    for i in &v {
        println!("{}", i);
    }

    // 修改元素
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

pub fn function_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    // 判断 数组是否为空
    let r = v.is_empty();
    println!("vector is empty: {r}");

    // 在指定 索引 插入数据
    v.insert(0, 6);

    // 移除指定位置的元素
    let value = v.remove(5);

    println!("{value}");

    // 删除尾部元素并返回 Option 枚举值
    let value = v.pop();

    println!("{value:?}");

    // 清空数组
    v.clear();

    // 附加数据 到 v
    v.extend([111, 33, 4]);

    println!("{:?}", v);

    // 静态数组转 动态数组
    let mut v1 = [11, 22].to_vec();
    // v1 附加在 v中，v1被清空
    v.append(&mut v1);
    println!("{:?}", v);
    println!("{:?}", v1);

    // 截断 到指定长度
    v.truncate(1);

    // 保留满足条件的元素
    v.retain(|x| *x > 10);
    println!("{:?}", v);

    let mut m = vec![11, 22, 33, 44, 55];

    // m 删除指定位置的元素，同时获取被删除元素的迭代器
    let n: Vec<_> = m.drain(1..3).collect();

    println!("{:?}", m); // [11, 44, 55]
    println!("{:?}", n); // [22, 33]

    // 分割数组 在指定索引处
    let m1 = m.split_off(1);
    println!("{:?}", m); // [11]
    println!("{:?}", m1); // [44, 55]
}

pub fn sort_vec() {
    let mut v = vec![11, 2, 34, 4, 15, 8];

    v.sort();
    println!("{:?}", size_of(&v));

    let mut f = vec![4.2, 2.3, 3.4, 3.5];

    // 对浮点数进行排序，浮点数存在 NAN， 没有实现 全数值可比较的 Ord 特征，
    // 若确定不含NAN，可使用 partial_cmp 判断大小
    f.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", f);
}
