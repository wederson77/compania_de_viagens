mod viagens;
use viagens::{DadosPassageiro, DadosVoos, adcionar_passageiro, adcionar_voo, exibir_passageiros, exibir_voos};

    //Crie um programa principal
fn main() {
    let mut dados_passageiros: Vec<DadosPassageiro> = Vec::new();
    let mut dados_voos: Vec<DadosVoos> = Vec::new();

    //Adcione alguns passageiros
    adcionar_passageiro(&mut dados_passageiros, String::from("João"), String::from("123ABC"), 18);
    adcionar_passageiro(&mut dados_passageiros, String::from("Wenderson"), String::from("777CCB"), 77);
    adcionar_passageiro(&mut dados_passageiros, String::from("Liliane"), String::from("7070LLL"), 25);

    //Adcionar alguns voos
    adcionar_voo(&mut dados_voos, String::from("Voo101"), String::from("São Paulo"), String::from("Rio de Janeiro"), String::from("30/06/2025"), String::from("09:00"));
    adcionar_voo(&mut dados_voos, String::from("Voo102"), String::from("Rio de Janeiro"), String::from("São Paulo"), String::from("30/12/2025"), String::from("11:00"));

    //Exibir os passageiros
    exibir_passageiros(&dados_passageiros);

    //Exiir os voos
    exibir_voos(&dados_voos);
}
