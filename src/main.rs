fn main() {
    println!("Hello, world!");    
    fungsi_baru();
    fungsi_argumen(34.567);
    fungsi_argumen2(-200, 'F');
    state_expresi();
    let _angka = angka();
    println!("angka return values: {_angka}");
    let _jumlah = penjumlahan(255);
    println!("penjumlahan return value: {_jumlah}");
}

fn fungsi_baru(){
    let _x : u8 = 5; //statment
    let _y : u8 = 20;
    let _jml: u8 = _x * _y;
    println!("fungsi baru = {_jml}");
}

// parameter (fungsi dengan argumen)
fn fungsi_argumen(_z: f32){
    println!("nilai parameter: {_z}");
}

fn fungsi_argumen2(_a: i32, _b: char) {
    println!("nilainya : {_a}, {_b}");
}

// statement dan expresion
fn state_expresi() {
    let _c: u8 = {
        let _d : u8 = 4; // ********* expression
        _d + 1 // ********** expression
    }; // cara menulis expresion berbeda
    println!("nilai c adalah {_c}");
}

// function with return values
fn angka() -> u16 {
    1_000
}

fn penjumlahan(_e: u16) -> u16 {
    _e + 255    
}