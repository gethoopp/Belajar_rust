mod basic;
#[tokio::main]
async fn main() {
    // Jalankan server dari routes
    // learn_rust::run_server().await;
    basic::data_types::tuple::tuple_demo();
    basic::data_types::char::char_demo();
    basic::data_types::array::array_datas();
    basic::data_types::constant::constant_data();
    basic::data_types::string_type::trim_string();
    basic::control_flow::if_expression::if_expression();
    basic::control_flow::loop_expression::looping();
}








