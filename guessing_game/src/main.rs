use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("##dev mode: El número secreto es:{}\n", secret_number);

    println!("Adivina el número!!");
    println!("Por favor dí un número: ");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Eror al leer");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pon un número mamonazo!!");
                continue;
            }
        };

        println!("Has dicho: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado pequeño"),
            Ordering::Greater => println!("Demasiado Alto"),
            Ordering::Equal => {
                println!("Ganas!!!");
                break;
            }
        }
    }
}
