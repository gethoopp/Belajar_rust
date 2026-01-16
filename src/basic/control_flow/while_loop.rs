// while loop jenis perulangan yang memiliki kondisi 
// di mana jika kondisi belum terpenuhi maka perulangan akan tetap dilanjutkan 


pub fn tes_whileLopp() {
    let mut nilaiUjian = 0;

    while nilaiUjian <=50 {
        if nilaiUjian >= 45 {
            println!("nilai ujian anda memenuhi KKM {}",nilaiUjian);
            // break;
        }
        if nilaiUjian < 45 {
            println!("nilai ujian anda {}",nilaiUjian);
        }
     nilaiUjian += 1   
    }
    
     
}