pub mod compound_type;
pub mod define_display;
pub mod define_func;
pub mod define_variable;
pub mod hello_world;
pub mod num_library;
pub mod operate_string;
pub mod ownership;
pub mod range_type;
pub mod reference_borrowing;
pub mod scalar_type;
pub mod type_convert;
pub mod utils;
fn main() {
    println!("Start Rust:");

    // hello_world::greet_world();

    // define_display::display_complex();

    // define_variable::def_variable();

    // define_variable::parse_variable();

    // scalar_type::def_int_type();

    // scalar_type::def_float_type();

    // scalar_type::numeric_operate();

    // scalar_type::bit_operate();

    // range_type::range_number();

    // range_type::range_char();

    // type_convert::u16_as_i32();

    // num_library::add_complex();

    // scalar_type::char_type();

    // scalar_type::unit_type();

    // define_func::statement_expression();

    // ownership::ownership_ins();

    // reference_borrowing::ref_bor();

    // reference_borrowing::read_string_len();

    // reference_borrowing::change_string_data();

    // reference_borrowing::mut_reference();

    // compound_type::remove_warning();

    // compound_type::read_slice();

    // operate_string::push_string();

    // operate_string::insert_string();

    operate_string::replace_string();
}
