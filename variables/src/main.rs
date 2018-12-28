fn main() {
    let variable: i32 = 10;
    let pointer: &i32 = &variable;

    println!("\nmemoria: {:p}, valor:{}", pointer, variable);

    let variable = 20;
    let pointer2:&i32 = &variable;

    println!("\nmemoria: {:p}, valor:{}", pointer2, variable);
    println!("original {}",pointer);
}
