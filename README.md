# 🦀 Belajar Rust

Repositori ini berisi catatan, contoh kode, dan latihan pribadi untuk mempelajari **Rust Programming Language** dari dasar hingga konsep lanjutan.  
Rust dikenal sebagai bahasa yang **aman, cepat, dan modern**, dengan fokus pada *memory safety* tanpa garbage collector.

---

## 📌 Tujuan

- Memahami dasar bahasa Rust
- Menguasai konsep ownership, borrowing, dan lifetimes
- Mempelajari error handling idiomatik Rust
- Mengenal concurrency & async programming di Rust
- Membiasakan struktur project Rust dengan Cargo
- Menjadi referensi pribadi dan arsip belajar

---

## 🛠️ Prasyarat

- Rust (stable) terbaru
- Cargo (terinstall otomatis bersama Rust)
- Text editor / IDE (VS Code + Rust Analyzer disarankan)
- Basic pemahaman pemrograman

Cek versi:
```bash
rustc --version
cargo --version


## 🚀 Instalasi Rust --> Menggunakan rustup (direkomendasikan):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


## Struktur Folder

belajar-rust/
├── basic/
│   ├── hello-world/
│   ├── variable/
│   ├── data-types/
│   ├── control-flow/
│   └── function/
├── ownership/
│   ├── ownership/
│   ├── borrowing/
│   └── lifetime/
├── intermediate/
│   ├── struct/
│   ├── enum/
│   ├── pattern-matching/
│   ├── error-handling/
│   └── module/
├── concurrency/
│   ├── thread/
│   ├── mutex/
│   ├── channel/
│   └── atomic/
├── async/
│   ├── async-await/
│   └── tokio/
├── database/
│   ├── postgres/
│   └── query/
└── README.md
