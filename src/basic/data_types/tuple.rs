//Tuple adalah kumpulan tipe data lebih dari satu tipe data, jumlah dalam tuple bersifat final dan tidak bisa diubah lagi jumlahnya 
// data dalam tuple dapat berbeda beda 
//untuk mengakses tiap data dalam tuple dapat menggunakan . dengan diikuti lokasi index yang di mana diawali dengan nomor 0 


pub fn tuple_demo() {
    let data_tuple: (i8, String, char) = (25, String::from("Haliim Pamungkas"), 'a');
    println!("{:?}",data_tuple);
    println!("{:?}", data_tuple.0); //mengambil index tuple ke - 0 
    println!("{:?}", data_tuple.1); // mengambil index tuple ke - 1 
    println!("{:?}", data_tuple.2); // mengambil index tuple ke - 2 

    //destructuring tuple
    //kadang ketika ingin menyimpan seluruh data di tuple akan menyulitkan jika akses satu satu 
    //jika ada data yang tidak dibutuhkan maka dapat ditambahkan _ pada data yang tidak dibutuhkan 


    let (data1,_,data3) = data_tuple;
    println!("{} {}",data1,data3);

    // data tuple pada default nya bersifat imuutable atua tidak dapat diubah namun kita dapat mengubahnya dengan menambahkan mut sebelum nama variable 
    //unit adalah tuple tanpa pnuya nilai apapaun atau kosong -> biasanya digunakan di function yang tidak mengembalikan apapun
    let mut data_tuples: (i8, String, char) = (25, String::from("Haliim Pamungkas"), 'a');

    data_tuples.1 = String::from("Harjo Suyono");

    println!("{:?}", data_tuples);
 
}