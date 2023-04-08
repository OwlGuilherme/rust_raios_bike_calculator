mod funcoes;
use funcoes::calc_camp_raio;
use funcoes::calc_fator_flange;
use std::io;

fn main() {
    let mut erd = String::new();
    let mut diam_aro = String::new();
    let mut diam_cubo = String::new();
    let mut num_furos = String::new();
    let mut padrao_cruz = String::new();

    println!("--- Calculadora de raios ---");

    println!("Digite o ERD da roda: ");
    io::stdin().read_line(&mut erd).expect("Falha ao ler entrada");
    let erd: f32 = match erd.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Entrada inválida"),
    };

    println!("Digite o diâmetro do aro: ");
    io::stdin().read_line(&mut diam_aro).expect("Falha ao ler entrada");
    let diam_aro: f32 = match  diam_aro.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Entrada inválida"),
    };

    println!("Digite o diâmetro do cubo da bicicleta: ");
    io::stdin().read_line(&mut diam_cubo).expect("Falha ao ler entrada");
    let diam_cubo: f32 = match diam_cubo.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Entrada inválida"),
    };

    println!("Digite o número de furos do cubo: ");
    io::stdin().read_line(&mut num_furos).expect("Falha ao ler entrada");
    let num_furos: i32 = match num_furos.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Entrada inválida"),
    };

    println!("Digite o padrão de variação: ");
    io::stdin().read_line(&mut padrao_cruz).expect("Falha ao ler entrada");
    let padrao_cruz: i64 = match padrao_cruz.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Entrada inválida")
    };



    println!("ERD: {}", erd);
    println!("Diâmetro do cubo: {}", diam_cubo);
    println!("Diâmetro do aro: {}", diam_aro);
    println!("Número de furos do cubo: {}", num_furos);
    println!("Padrão de cruzamento do aros: {}", padrao_cruz);
}
