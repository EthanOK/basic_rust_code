use std::collections::HashMap;

use crate::basic::utils::vec_to_array;

/**
 * HashMap
 */

pub fn create_hashmap() {
    // new()
    let mut map = HashMap::new();
    map.insert("A", 2);
    map.insert("B", 3);
    map.insert("C", 4);
    println!("{:?}", map);

    // 通过索引返回 value
    let vv = map["B"];
    println!("{}", vv);

    // iter 和 collect 创建

    let team_list = vec![("CHN", 10), ("JP", 5), ("USA", 6)];

    // into_iter 将数组转为 迭代器，再通过 collect 收集，需要显式标注 类型
    let team_map: HashMap<_, _> = team_list.clone().into_iter().collect();

    println!("{:?}", team_map);

    let team_list1 = [("CHN", 10), ("JP", 5), ("USA", 6)];

    let team_map2 = HashMap::from(team_list1);
    println!("{:?}", team_map2);

    let team_list2 = vec_to_array(team_list);

    assert_eq!(team_list2, team_list1);
}

pub fn select_hashmap() {
    let mut map = HashMap::new();
    map.insert("A", 2);
    map.insert("B", 3);
    map.insert("C", 4);

    // get() 返回一个 Option<&T>
    let v = map.get("A");

    // copied() 将 Option<&T> 转换为 Option<T>
    let v = map.get("A").copied();

    // unwrap_or_default() 将 Option<T> 转换为 T，如果为 None 则返回默认值

    let v = v.unwrap_or_default();
    println!("{}", v);

    if let Some(value) = map.get("A") {
        println!("{}", value);
    }

    // get_mut
    if let Some(value) = map.get_mut("A") {
        *value = 10;
    }

    for (k, v) in &map {
        println!("{} {}", k, v);
    }
}

pub fn update_hashmap() {
    let mut map = HashMap::new();
    map.insert("A", 2);
    map.insert("B", 3);
    map.insert("C", 4);

    // insert() 插入数据，如果 key 已经存在，则更新 value
    map.insert("A", 10);

    // 查询 “A” 对应的值，若不存在则插入新值
    // or_insert() 返回一个 &mut T，这个 T 必须是可变的，否则编译器会报错
    let a = map.entry("A").or_insert(100);
    println!("{}", a);

    let d = map.entry("D").or_insert(100);
    // *d 解引用，再更新
    *d = 200; // 更新 d 的值为 200
    println!("{}", d);

    println!("{:?}", map);
}
