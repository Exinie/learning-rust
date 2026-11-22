use rand::RngExt;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    let tahmin_edilecek = rand::rng().random_range(0..=100);
    let tahmin: i32;

    if args.len() > 1 {
        debug = true;
    }

    if debug {
        println!("[DEBUG] {}", tahmin_edilecek);
    }

    print!("Tahmin girin: ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).expect("Hata!");

    match girdi.trim().parse::<i32>() {
        Ok(tahmin) => {
            if tahmin == tahmin_edilecek {
                println!("Tahmin doğru.");
            } else {
                println!("Tahmin yanlış!");
            }
        }
        Err(_) => {
            println!("Geçersiz girdi!");
            return ();
        }
    }
}
