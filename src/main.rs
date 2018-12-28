use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Adivina el número!!");
	println!("Por favor dí un número: ");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	
	println!("El número secreto es:{}",secret_number);

	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess)
		.expect("Eror al leer");

	let guess: u32 = guess.trim().parse()
		.expect("Por favor pon un número");

	println!("Has dicho: {}",guess);

	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Demasiado pequeño"),
		Ordering::Greater => println!("Demasiado Alto"),
		Ordering::Equal => println!("Ganas!!!"),
	}
}
