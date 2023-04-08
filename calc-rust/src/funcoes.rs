pub fn fator_flange(num_furos: i32, padrao_cruz: f64) -> f64 {
    let angulo = 360.0 / (num_furos as f64 * 0.5 * padrao_cruz);
    let fator_flange = angulo.to_radians().cos();
    fator_flange
}

pub fn camp_raio(){
    println!("Comprimento do raio");
}
