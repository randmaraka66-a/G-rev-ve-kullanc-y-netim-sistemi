//?sprint1
// use std::io;

// struct Task {
//     title: String,
//     done: bool,
// }

// fn main() {
//     let mut tasks: Vec<Task> = Vec::new();

//     loop {
//         println!("\n--- MENU ---");
//         println!("1 - Görev Ekle");
//         println!("2 - Görevleri Listele");
//         println!("3 - Çıkış");

//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).unwrap();

//         match choice.trim() {


//             "1" => {
//                 println!("Görev adını gir:");

//                 let mut title = String::new();
//                 io::stdin().read_line(&mut title).unwrap();

//                 let task = Task {
//                     title: title.trim().to_string(),
//                     done: false,
//                 };

//                 tasks.push(task);

//                 println!("Görev eklendi!");
//             }


//             "2" => {
//                 if tasks.is_empty() {
//                     println!("Görev bulunamadı.");
//                 } else {
//                     for (i, task) in tasks.iter().enumerate() {
//                         println!("{} - {}", i, task.title);
//                     }
//                 }
//             }


//             "3" => {
//                 println!("Program kapatılıyor...");
//                 break;
//             }

//             _ => {
//                 println!("Geçersiz seçim!");
//             }
//         }
//     }
// }

//? sprint2
// use std::io;

// struct Task {
//     title: String,
//     done: bool,
// }

// fn main() {
//     let mut tasks: Vec<Task> = Vec::new();

//     loop {
//         println!("\n===== GÖREV YÖNETİM SİSTEMİ =====");
//         println!("1 - Görev Ekle");
//         println!("2 - Görevleri Listele");
//         println!("3 - Görev Sil");
//         println!("4 - Çıkış");

//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).unwrap();

//         match choice.trim() {
//             "1" => {
//                 println!("Görev adını giriniz:");

//                 let mut title = String::new();
//                 io::stdin().read_line(&mut title).unwrap();

//                 tasks.push(Task {
//                     title: title.trim().to_string(),
//                     done: false,
//                 });

//                 println!("Görev başarıyla eklendi.");
//             }

//             "2" => {
//                 if tasks.is_empty() {
//                     println!("Kayıtlı görev bulunamadı.");
//                 } else {
//                     println!("\nGörev Listesi:");

//                     for (i, task) in tasks.iter().enumerate() {
//                         println!("{} - {}", i + 1, task.title);
//                     }
//                 }
//             }

//             "3" => {
//                 if tasks.is_empty() {
//                     println!("Silinecek görev bulunamadı.");
//                     continue;
//                 }

//                 println!("Silmek istediğiniz görev numarasını giriniz:");

//                 let mut index = String::new();
//                 io::stdin().read_line(&mut index).unwrap();

//                 let index: usize = match index.trim().parse() {
//                     Ok(num) => num,
//                     Err(_) => {
//                         println!("Geçersiz giriş!");
//                         continue;
//                     }
//                 };

//                 if index == 0 || index > tasks.len() {
//                     println!("Geçersiz görev numarası!");
//                 } else {
//                     tasks.remove(index - 1);
//                     println!("Görev silindi.");
//                 }
//             }

//             "4" => {
//                 println!("Program sonlandırıldı.");
//                 break;
//             }

//             _ => println!("Geçersiz seçim yaptınız."),
//         }
//     }
// }

//?sprint3
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
        println!("4 - Görev Tamamla");
        println!("5 - Çıkış");

        let choice = get_input();

        match choice.as_str() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => delete_task(&mut tasks),
            "4" => complete_task(&mut tasks),
            "5" => {
                println!("Program sonlandırıldı.");
                break;
            }
            _ => println!("Geçersiz seçim!"),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Görev adını giriniz:");
    let title = get_input();

    tasks.push(Task {
        title,
        done: false,
    });

    println!("Görev eklendi.");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("Görev yok.");
        return;
    }

    println!("\nGörev Listesi:");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done { "✔" } else { "✖" };
        println!("{} - {} [{}]", i + 1, task.title, status);
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Silinecek görev numarası:");
    let index = parse_index();

    if index == 0 || index > tasks.len() {
        println!("Geçersiz numara!");
        return;
    }

    tasks.remove(index - 1);
    println!("Görev silindi.");
}

fn complete_task(tasks: &mut Vec<Task>) {
    println!("Tamamlanacak görev numarası:");
    let index = parse_index();

    if index == 0 || index > tasks.len() {
        println!("Geçersiz numara!");
        return;
    }

    tasks[index - 1].done = true;
    println!("Görev tamamlandı.");
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_index() -> usize {
    let input = get_input();

    match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz giriş!");
            0
        }
    }
}
