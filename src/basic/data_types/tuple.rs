//Tuple adalah kumpulan tipe data lebih dari satu tipe data, jumlah dalam tuple bersifat final dan tidak bisa diubah lagi jumlahnya 
// data dalam tuple dapat berbeda beda 
//untuk mengakses tiap data dalam tuple dapat menggunakan . dengan diikuti lokasi index yang di mana diawali dengan nomor 0 


pub fn tuple_demo() {
    let data_tuple: (i8, String, char) = (25, String::from("Haliim Pamungkas"), 'a');
    println!("{:?}",data_tuple);
    println!("{:?}", data_tuple.0) //mengambil index tuple ke - 0 
    println!("{:?}", data_tuple.1) // mengambil index tuple ke - 1 
    println!("{:?}", data_tuple.2) // mengambil index tuple ke - 2 

    //destructuring tuple
}