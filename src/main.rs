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
    // basic::control_flow::loop_expression::looping();
    basic::control_flow::while_loop::tes_whileLopp();
    basic::control_flow::for_loop::contohhLopp();
   let nums = vec![3, 1, 4, 7, 2];
   let nums2 = vec![4, 2, 1, 3];
   println!("{:?}",basic::control_flow::for_loop::find_missing(nums,nums2));
   learn_rust::contoh_owneship();
    


    
}








