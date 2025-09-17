// src/main.rs

fn main() {
    // 1. Criamos um vetor para armazenar os níveis de expressão de vários genes.
    // Usamos a macro 'vec!' para iniciar.
    let mut niveis_expressao = vec![150.5, 200.0, 50.2, 95.8, 310.0];

    println!("--- Análise de Expressão Gênica ---");
    println!("Valores de expressão orgiginais: {:?}", niveis_expressao);

    // 2. Chamamos uma função que lê os valores para calcular a média.
    // Passamos uma referência imutável (&).
    let media = calcular_media(&niveis_expressao);
    println!("A média de expressão é: {:.2}", media);

    // 3. Chamamos uma função que MODIFICA os valores para normalizá-los.
    // Passamos uma referência mutável (&mut).
    normalizar_valores(&mut niveis_expressao);
    println!("Valores de expressão normalizados: {:?}", niveis_expressao);

    // 4. Acessando um valor de forma segura com o .get()
    match niveis_expressao.get(2) {
        Some(valor) => println!("O terceiro valor é: {:.2}", valor),
        None => println!("Não foi possível encontrar o terceiro valor."),
    }
}

/// Calcula a média de um vetor de f64.
/// Recebe um empréstimo IMUTÁVEL, pois só precisa ler os dados.
fn calcular_media(valores: &Vec<f64>) -> f64 {
    let mut soma = 0.0;
    //Iteramos sobre cada valor para somá-los.
    for valor in valores { // 'valores' aqui já é uma referência.
        soma += valor;
    }
    soma / valores.len() as f64 // Retornamos a média.
}

/// Divide cada valor no vetor por 10.0 para "normalizá-los".
/// Recebe um empréstimo MUTÁvel, pois precisa alterar os dados.
fn normalizar_valores(valores: &mut Vec<f64>) {
    // Iteramos sobre cada valor com uma referência mutável.
    for valor in valores {
        // Usamos o operador de desreferência '*' para alterar o valor para o qual a referência aponta.
        *valor = *valor / 10.0;
    }
}
