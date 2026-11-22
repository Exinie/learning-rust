use std::io::{self, Write};

struct Medya {
    baslik: String,
    tur: Tur,
    puan: i32,
}

#[derive(Debug)]
enum Tur {
    Series,
    Movie,
    Anime,
}

fn main() {
    let example_data = Medya {
        baslik: "Promare".to_string(),
        tur: Tur::Anime,
        puan: 10,
    };

    let mut list = vec![example_data];

    loop {
        print!("Başlık: ");
        io::stdout().flush().unwrap();

        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("Hata!");

        print!("Tür: ");
        io::stdout().flush().unwrap();

        let mut media_type = String::new();
        io::stdin().read_line(&mut media_type).expect("Hata!");

        print!("Puan: ");
        io::stdout().flush().unwrap();

        let mut score = String::new();
        io::stdin().read_line(&mut score).expect("Hata!");

        let media_type_enum = match media_type.trim() {
            "dizi" | "series" => Tur::Series,
            "film" | "movie" => Tur::Movie,
            "anime" => Tur::Anime,
            _ => {
                println!("Geçersiz tür girdiniz!");
                continue;
            }
        };

        match score.trim().parse::<i32>() {
            Ok(score_converted) => {
                if score_converted <= 10 && score_converted >= 1 {
                    let new_data = Medya {
                        baslik: title.trim().to_string(),
                        tur: media_type_enum,
                        puan: score_converted,
                    };

                    list.push(new_data);
                } else {
                    println!("Lütfen 1 ile 10 arasında bir puan giriniz.");
                    continue;
                }
            }
            Err(_) => {
                println!("Geçersiz girdi!");
                // return ();
                continue;
            }
        }

        println!("--- MEDIA LIST ---");

        for medya in &list {
            println!(
                "Başlık:{}\nTür:{:#?}\nPuan:{}\n",
                medya.baslik, medya.tur, medya.puan
            );
        }

        println!("--- END MEDIA LIST ---");
        println!("")
    }
}
