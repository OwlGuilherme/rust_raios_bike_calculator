mod funcoes;
use funcoes::camp_raio;
use funcoes::fator_flange;
use std::io;

fn main() {
    println!("--- Calculadora de raios ---");
    println!("Digite o ERD da roda: ");
    let mut erd = String::new();

    io::stdin().read_line(&mut erd)
        .expect("Falha ao ler entrada");

    println!("ERD: {}", erd);
}
