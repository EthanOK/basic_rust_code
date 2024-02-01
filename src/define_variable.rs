pub fn def_variable() {
    // 变量 默认不可变
    let immmut_a = 100;
    println!("屏蔽前 immmut_a is {}", immmut_a);

    // 需使用 mut 关键字 声明可变变量
    let mut mut_a = 900;

    println!("修改前 mut_a is {}", mut_a);
    mut_a = 9999;

    println!("修改后 mut_a is {}", mut_a);

    // 使用下划线开头忽略未使用变量
    let _ddd = 333;

    // 常量 必须指定类型 全部大写
    #[allow(dead_code)]
    const MAX_NUMBER: i32 = 1000;

    // 对之前的变量进行屏蔽，重新定义，数据类型可以 不相同
    let immmut_a = 2.5;

    {
        let immmut_a = immmut_a * 2.0;

        println!("作用域内 immmut_a is {}", immmut_a);
    }
    println!("屏蔽后 immmut_a is {}", immmut_a);
}

struct Demo<'a> {
    e: &'a str,
    g: i32,
}
pub fn parse_variable() {
    let (a, b) = (188, true);

    println!("a = {}, b = {} ", a, b);

    let (c, d, e);

    [c, .., d, _] = [1, 2, 3, 4, 5, 6];
    // c 是第一个元素
    // d 是倒数第二个元素

    Demo { e, .. } = Demo { e: "helllo", g: 22 };

    println!("c = {}, d = {}, e = {} ", c, d, e);
}
