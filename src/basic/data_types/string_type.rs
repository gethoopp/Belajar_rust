/* 
Rust memiliki tipe data text yang fix yaitu &str dan yang dapat berkembang adalah String */


pub fn trim_string() {
    //String Slice 
    let trimString: &str = " Pamungkas Harjo ";
    let fixString: &str = trimString.trim();

    println!("{}",trimString);
    println!("{}",fixString);

    //Tipe data String
   let mut data = String::from("Belajar");
    let r1 = &data;
    println!("{}", r1);
    let r2 = &mut data;
    r2.push_str(" Rust");

    println!("{}", data);
}


// pub fn first_word(n: &str): &str {

// }