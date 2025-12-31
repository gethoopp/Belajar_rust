//Rust mendukung beberpaa cara untuk menggunakan perulanagan termasuk loop


pub fn looping() {
    let mut number = 0;
    let result= loop {
          number += 1 ;

        if number >= 10 {
            break;
        }
    };

    println!("{:?}", number);



    let mut increment =0;

    loop{
        if increment <= 0 {
            increment+=1;
        } else if increment > 10 {
            break;
        }
    }

    println!("{}", increment);
}