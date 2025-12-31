
/* 
Constant merupakan variable yang immutable dengan kata kunci const constant tidak punya vairabel yang mutable dan harus dibuat saat kode dibuat bukan saat kode dijalankan 
untuk membuat constant wajib menyebutkan tipe data secara eksplisit


*/



const NILAI: i32 = 50;

pub fn constant_data() {
  println!("{:?}",NILAI);
}