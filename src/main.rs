use std::error::Error;

use rust_course::basic::{
    compound_type, control_flow, define_display, define_func, define_method, define_variable,
    dyn_array, format_rust, generics_type, hashmap, hello_world, lifetime, num_library,
    operate_string, ownership, pattern_match, range_type, reference_borrowing, result_error,
    scalar_type, trait_rust, type_convert, use_mod,
};

fn main() -> Result<(), Box<dyn Error>> {
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

    // operate_string::replace_string();

    // operate_string::delete_string();

    // operate_string::concatenate_string();

    // operate_string::escape_string();

    // operate_string::operate_utf8();

    // compound_type::tuple_type();

    // compound_type::enum_type();

    // compound_type::array_type();

    // compound_type::struct_type();

    // compound_type::tuple_struct();

    // control_flow::if_control();

    // control_flow::for_control();

    // pattern_match::match_demo();

    // pattern_match::matches_demo();

    // pattern_match::match_mut_ref();

    // define_method::caculate_area_struct();

    // define_method::call_enum();

    // println!("value: {}", generics_type::add(2.6, 7.4));

    // let array = [3, 5, 6, 9, 99, 8, 14, 6, 0];

    // println!("largest value: {}", generics_type::largest(&array));

    // generics_type::struct_generics();

    // generics_type::func_generics();

    // generics_type::print_array();

    // trait_rust::trait_bound();

    // trait_rust::trait_object();

    // dyn_array::create_vec();

    // dyn_array::read_element_vec();

    // dyn_array::function_vector();

    // dyn_array::sort_vec();

    // hashmap::create_hashmap();

    // hashmap::select_hashmap();

    // hashmap::update_hashmap();

    // lifetime::function_lifetime();

    // lifetime::struct_lifetime();

    // result_error::panic_demo();

    // result_error::result_demo();

    // result_error::panic_unwrap_demo();

    // result_error::panic_expect_demo();

    // result_error::wenhao_demo();

    // use_mod::get_random(1, 200);

    format_rust::format_to_string();

    format_rust::format_struct();

    Ok(())
}
