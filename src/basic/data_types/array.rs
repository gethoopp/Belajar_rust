/* 
Array adalah kumpulan data yang hanya dapat memuat satu tipe data, dan membuat array 
harus menggunakan kurung kotak [] array di rust ukurannya harus fix ditentukan 
sebelum compile run
*/

pub fn array_datas() {
    //imutable array
    let array_data: [i32;5] = [1,2,3,4,5];
    println!("{:?}",array_data);


    //muttable array
    let mut array_datas: [i32;5] = [1,2,3,4,5];

    array_datas[0] = 20;
    array_datas[1] = 50;

    println!("{:?}", array_datas);
    println!("{} {}",array_data[1],array_data[2]);


    //two dimensional array 
    let two_dimensional_array: [[i32;2];2] = [
        [1,3], 
        [4,5]];
    println!("{:?}", two_dimensional_array);
    println!("{} {}", two_dimensional_array[0][1], two_dimensional_array[1][0])
}