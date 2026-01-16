pub fn if_expression() {
    let value = 8;

    // if value >= 10 {
    //     println!("GOOD RESULT")
    // }else {
    //     println!("TRY AGAIN")
    // }


    let result: i32 = if value >= 10 {
        10
    } else if value >= 6 {
        9
    } else {
        0
        
    };

    println!("{}", result);
}