mod funcoes;
use funcoes::camp_raio;
use funcoes::fator_flange;
use std::io;

fn main() {
    let mut erd = String::new();
    let mut diam_cubo = String::new();
    let mut num_furos = String::new();
    let mut padrao_cruz = String::new();

    println!("--- Calculadora de raios ---");
    println!("Digite o ERD da roda: ");

    io::stdin().read_line(&mut erd)
        .expect("Falha ao ler entrada");

    println!("ERD: {}", erd);
}
