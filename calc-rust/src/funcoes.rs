pub fn calc_fator_flange(num_furos: i32, padrao_cruz: f64) -> f64 {
    let angulo = 360.0 / (num_furos as f64 * 0.5 * padrao_cruz);
    let fator_flange = angulo.to_radians().cos();
    fator_flange
}

pub fn calc_camp_raio(diam_raio: f64, diam_cubo: f64, fator_flange: f64) -> f64 {
    let comp_raio = ((diam_raio / 2.0).powi(2) + (diam_cubo / 2.0).powi(2) - (fator_flange * diam_cubo / 2.0 ).powi(2)).sqrt();
    comp_raio
}
