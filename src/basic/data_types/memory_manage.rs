/* 

Dalam rust memiliki pendekatan berbeda, rust tidak menggunakan garbage collection, rust juga tidak menggunkaan manual memory management rust membagi ke dalam dua bagian stack dan heap stack merupakan bagian data yang didalamnya berada dalam tumpuk dengan metode LIFO dan data di dalamnya ukurannya sudah pasti heap merupakan tempat menyimpan data yang didalamnya terdapat memory allocator
Drop function merupakan function untuk menghapus data dari heap maupun stack, saat rust sudah mengeskesuksi suatu function maka funciton tersebut langsung dihapus dari stack frame nya 

*?