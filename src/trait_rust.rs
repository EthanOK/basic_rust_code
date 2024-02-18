/**
 * 范型 + 特征约束 ：静态分发 编译期完成
 * 特征对象：动态分发 运行时才确定调用什么方法。
 * 特征对象的大小不固定，但特征对象的引用类型大小固定，2 个指针大小。
 *
 * 特征对象的限制：不是所有的特征都有特征对象，Clone 就没有
 * （1）方法的返回类型不能是Self
 * （2）方法没有任何范型参数
 */
pub trait Draw {
    fn draw(&self);
}

pub struct Demo {
    pub name: String,
}

impl Demo {
    fn draw(&self) {
        println!("{}", self.name);
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing button with label: {}", self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing select box with options: {:?}", self.options);
    }
}

/**
 * 特征约束
 * 范型 T 必须实现了特征 Draw
 *
 */
fn draw_item<T: Draw>(item: &T) {
    item.draw();
}

pub fn trait_bound() {
    let draw1 = Button {
        width: 100,
        height: 100,
        label: String::from("This is button"),
    };

    draw_item(&draw1);

    let demo = Demo {
        name: String::from("This is demo"),
    };
    // error :the trait `Draw` is not implemented for `trait_rust::Demo`
    // draw_item(&demo);
    demo.draw();

    let draw2 = SelectBox {
        width: 100,
        height: 100,
        options: vec![String::from("Option 1"), String::from("Option 2")],
    };
    draw_item(&draw2);
}

// error: (impl Trait 的返回值类型不支持多种类型返回)
// fn return_draw(switch: bool) -> impl Draw {
//     if switch {
//         Button {
//             width: 100,
//             height: 100,
//             label: String::from("This is button"),
//         }
//     } else {
//         SelectBox {
//             width: 100,
//             height: 100,
//             options: vec![String::from("Option 1"), String::from("Option 2")],
//         }
//     }
// }

/**
 * 特征对象 Box<dyn Draw>
 * 定义特征对象： x: &dny Draw 或 x: Box<dny Draw>
 */

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 使用 Box::new(T) 创建了两个 Box<dyn Draw> 特征对象
pub fn trait_object() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![String::from("Option 1"), String::from("Option 2")],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("This is button"),
            }),
        ],
    };
    screen.run();

    // error: the trait `Draw` is not implemented for `String`
    // let Screen = Screen {
    //     components: vec![Box::new(String::from("hello"))],
    // };
}
