    //Crie uma struct para armazenar os dados dos passageiros
pub struct DadosPassageiro{
    pub nome: String,
    pub numero_passaporte: String,
    pub idade: u8,
} 

    //Crie uma struct para armazenar os dados dos voos
pub struct DadosVoos{
    pub codigo_voo: String,
    pub partida: String,
    pub destino: String,
    pub data_partida: String,
    pub horario_partida: String,
}

    //Crie uma função para adcinar um novo passageiro
pub fn adcionar_passageiro(dados_passageiros: &mut Vec<DadosPassageiro>, nome: String, numero_passaporte: String, idade: u8){
    let passageiro = DadosPassageiro{nome, numero_passaporte, idade};
    dados_passageiros.push(passageiro);
}

    //Crie uma função para adcionar um novo voo
pub fn adcionar_voo(dados_voos: &mut Vec<DadosVoos>, codigo_voo: String, partida: String, destino: String, data_partida: String, horario_partida: String){
    let voo = DadosVoos{codigo_voo, partida, destino, data_partida, horario_partida};
    dados_voos.push(voo);
}

    //Crie uma função para exibir todos os voos
pub fn exibir_voos(dados_voos: &Vec<DadosVoos>){
    for voo in dados_voos{
        println!("Código do Voo: {}", voo.codigo_voo);
        println!("Partida: {}", voo.partida);
        println!("Desino: {}", voo.destino);
        println!("Data de Partida: {}", voo.data_partida);
        println!("Horario de Partida: {}\n", voo.horario_partida);
    }
}

    //Crie uma função para exibir todos os passageiros
pub fn exibir_passageiros(dados_passageiros: &Vec<DadosPassageiro>){
    for passageiro in dados_passageiros{
        println!("Nome: {}", passageiro.nome);
        println!("Numero de passaporte: {}", passageiro.numero_passaporte);
        println!("Idade: {}\n", passageiro.idade);
    }
}