use std::io;

fn main() {
    println!("Adivinhe um número!");

    println!("Digite o seu palpite: ");

    let mut palpite = String::new();

    io::stdin()
        .read_line(&mut palpite)
        .expect("Falha ao ler a entrada");
    println!(" Você disse: {}", palpite);
}
