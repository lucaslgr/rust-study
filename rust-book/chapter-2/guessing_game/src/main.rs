extern crate rand; //Especificando para o Rust que é uma dependência externa

use rand::Rng;
use std::cmp::Ordering;
use std::io; //Importando uma Trait que contem o método gen_range

fn main() {
    println!("Adivinhe um número!");

    

    loop {

        let numero_secreto = rand::thread_rng().gen_range(0..10);

        println!("Digite o seu palpite: ");

        let mut palpite = String::new();

        //OR io::Stdin::stdin().read_line....
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a entrada");
        // println!(" Você disse: {}", palpite);

        // let palpite: u32 = palpite.trim().parse().expect("Digite um número!");
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Você acertou!")
                break;
            },
        }

        // println!("O número secreto é: {}", numero_secreto);
    }
}
