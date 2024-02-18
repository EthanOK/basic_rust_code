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
