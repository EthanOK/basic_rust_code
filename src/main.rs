pub mod define_display;
pub mod define_variable;
pub mod hello_world;
pub mod value_type;
fn main() {
    println!("Start Rust:");

    // hello_world::greet_world();

    // define_display::display_complex();

    // define_variable::def_variable();

    // define_variable::parse_variable();

    // value_type::def_int_type();

    value_type::def_float_type();
}
