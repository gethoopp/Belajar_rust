//konsep ownership --> setiap value di rust harus memiliki owner (varaibel pemilik value) --> dalam satu waktu hanya boleh ada satu owner
//Namun data copy tidak terjadi untuk tipe data yang disimpan di heap
//Seperti aturan di ownership, dalam satu waktu value hanya memiliki satu owner 
//Saat mencoba variabel baru di data yang disimpan ke dalam heap seperti String, maka yang terjadi bukanlah copy data melainkan transfer ownership dari owner lama ke owner baru 
//Untuk melakukan copy pada data yang disimpan dalam heap, kita dapat menggunakan kata kunci clone



/* 
Owneship digunakna untuk mengelola data dalam emmory tanpa garbage colection
setiap value harus punya owner atau variabel pemegang nilai dalam satu waktu hanya boleh satu owner ketika owner keluar scope maka data di hapussemua data yang bersifat fixed size ketika ditamabhkan di vairabel yang berbeda maka data akan di copy bukan memindahkan data. 
*/



pub fn contoh_owneship() {
    let a:String = String::from("Haliim"); 

    {
       //nilai variabel a masih dapat di konsme di dalam blok ini 
       println!("{:?}",a);
       let nilai = 20;
    }
   // variabel nilai tidak bisa di konsume disini diakrenakan di luar scope
    // println!("{}",nilai);
    
}

