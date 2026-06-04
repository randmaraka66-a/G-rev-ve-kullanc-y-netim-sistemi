use std::io;

struct Task {
    title: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n===== GÖREV YÖNETİM SİSTEMİ =====");
        println!("1 - Görev Ekle");
        println!("2 - Görevleri Listele");
        println!("3 - Görev Sil");
        println!("4 - Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Görev adını giriniz:");

                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                tasks.push(Task {
                    title: title.trim().to_string(),
                    done: false,
                });

                println!("Görev başarıyla eklendi.");
            }

            "2" => {
                if tasks.is_empty() {
                    println!("Kayıtlı görev bulunamadı.");
                } else {
                    println!("\nGörev Listesi:");

                    for (i, task) in tasks.iter().enumerate() {
                        println!("{} - {}", i + 1, task.title);
                    }
                }
            }

            "3" => {
                if tasks.is_empty() {
                    println!("Silinecek görev bulunamadı.");
                    continue;
                }

                println!("Silmek istediğiniz görev numarasını giriniz:");

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Geçersiz giriş!");
                        continue;
                    }
                };

                if index == 0 || index > tasks.len() {
                    println!("Geçersiz görev numarası!");
                } else {
                    tasks.remove(index - 1);
                    println!("Görev silindi.");
                }
            }

            "4" => {
                println!("Program sonlandırıldı.");
                break;
            }

            _ => println!("Geçersiz seçim yaptınız."),
        }
    }
}