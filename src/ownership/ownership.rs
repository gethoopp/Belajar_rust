//konsep ownership --> setiap value di rust harus memiliki owner (varaibel pemilik value) --> dalam satu waktu hanya boleh ada satu owner
//Namun data copy tidak terjadi untuk tipe data yang disimpan di heap
//Seperti aturan di ownership, dalam satu waktu value hanya memiliki satu owner 
//Saat mencoba variabel baru di data yang disimpan ke dalam heap seperti String, maka yang terjadi bukanlah copy data melainkan transfer ownership dari owner lama ke owner baru 
//Untuk melakukan copy pada data yang disimpan dalam heap, kita dapat menggunakan kata kunci clone