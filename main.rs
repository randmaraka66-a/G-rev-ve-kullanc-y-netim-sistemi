
use std::io;

struct Task {
    title: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n1- Ekle\n2- Listele\n3- Sil\n4- Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Görev gir:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                tasks.push(Task {
                    title: title.trim().to_string(),
                    done: false,
                });

                println!("Eklendi!");
            }

            "2" => {
                for (i, task) in tasks.iter().enumerate() {
                    println!("{} - {}", i, task.title);
                }
            }

            "3" => {
                println!("Silinecek index:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let i: usize = index.trim().parse().unwrap_or(999);

                if i < tasks.len() {
                    tasks.remove(i);
                    println!("Silindi!");
                } else {
                    println!("Hatalı seçim!");
                }
            }

            "4" => break,

            _ => println!("Yanlış seçim"),
        }
    }
}