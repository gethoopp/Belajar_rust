mod base_rust;
pub use base_rust::print_text;
pub use base_rust::number_age;

mod api;
pub use api::routes::run_server;

mod basic;
pub use basic::data_types::tuple::tuple_demo;
pub use basic::data_types::char::char_demo;
pub use basic::control_flow::if_expression::if_expression;
pub use basic::control_flow::loop_expression::looping;
pub use basic::control_flow::while_loop::tes_whileLopp;
// pub use basic::control_flow::for_loop::plus_one;
pub use basic::control_flow::for_loop::find_missing;

mod ownership_function;
pub use ownership_function::ownership::contoh_owneship;


