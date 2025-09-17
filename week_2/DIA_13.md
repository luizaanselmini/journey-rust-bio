# Jornada Rust 🦀 em Bioinformática - DIA 13

## Vetores: Coleções de Dados Biológicos

Em nossa jornada até agora, trabalhamos principalmente com valores individuais: um gene, uma sequência, um conteúdo GC. Mas a bioinformática raramente lida com um único dado. Nós trabalhamos com *listas* de genes, *coleções* de leituras de sequenciamento, *conjuntos* de resultados de expressão.

Como podemos armazenar e gerenciar uma lista de itens em Rust? A resposta é a estrutura de dados mais comum para coleções: o **Vetor** (`Vec<T>`).

Pense em um vetor como uma placa de 96 poços em um laboratório. É um contêiner que pode guardar múltiplos itens (amostras), e você pode adicionar ou remover itens conforme a necessidade. Hoje, vamos aprender a criar e manipular essas "placas de poços" em nosso código para gerenciar coleções de dados biológicos.


### Objetivos do Dia

1.  **Entender** o que é um `Vec<T>` (Vetor).
2.  **Aprender** a criar, popular e modificar um vetor.
3.  **Descobrir** as duas formas de acessar elementos em um vetor: indexação e o método `.get()`.
4.  **Praticar** a iteração sobre os elementos de um vetor com um loop `for`.
5.  **Ver** como as regras de posse (ownership) se aplicam a vetores.


### Conceitos do Dia

#### O que é um Vetor?

Um `Vec<T>` (lê-se "Véc de T") é uma lista de tamanho variável onde todos os elementos devem ser do **mesmo tipo**. O `T` é um placeholder para o tipo de dado que o vetor irá armazenar (ex: `Vec<i32>` para uma lista de inteiros, `Vec<String>` para uma lista de strings). Assim como a `String`, os dados de um vetor são armazenados no *heap*, o que significa que ele pode crescer ou encolher de tamanho.

#### Criando um Vetor

Existem duas formas principais de criar um vetor:
```rust
// 1. Criar um vetor vazio e adicionar elementos depois.
// A variável precisa ser 'mut' para que possamos modificá-la.
let mut lista_scores = Vec::new();
lista_scores.push(10);
lista_scores.push(25);

// 2. Usar a macro 'vec!' para criar um vetor com valores iniciais.
// O Rust infere o tipo (neste caso, Vec<&str>).
let lista_genes = vec!["TP53", "BRCA1", "EGFR"];
```

#### Modificando um Vetor

* `.push(valor)`: Adiciona um elemento ao final do vetor.
* `.pop()`: Remove e retorna o último elemento do vetor (dentro de um `Option`).

#### Acessando Elementos

Existem duas maneiras de acessar um elemento em um vetor, com uma importante diferença de segurança:

1.  **Indexação (`[]`):** A forma mais direta.
    ```rust
    let primeiro_gene = &lista_genes[0]; // Acessa o primeiro elemento
    ```
    **Cuidado:** Se você tentar acessar um índice que não existe (ex: `lista_genes[99]`), seu programa entrará em **`panic!`** e será encerrado.

2.  **Método `.get()`:** A forma mais segura.
    ```rust
    let talvez_segundo_gene = lista_genes.get(1); // Tenta acessar o segundo elemento
    ```
    O método `.get()` não causa `panic!`. Em vez disso, ele retorna um `Option<&T>`:
    * `Some(&valor)` se o elemento existir.
    * `None` se o índice estiver fora dos limites.
    Isso te permite usar um `match` para lidar com a possibilidade de um elemento não existir de forma segura.

#### Iterando sobre um Vetor

A forma mais fácil de percorrer um vetor é com um loop `for`, que vimos no Dia 6.

```rust
let lista_scores = vec![10, 25, -5, 30];

// Empréstimo imutável para ler cada score
for score in &lista_scores {
    println!("Score: {}", score);
}

// Empréstimo mutável para modificar cada score
let mut lista_mutavel = vec![100, 200, 300];
for valor in &mut lista_mutavel {
    *valor += 10; // O '*' é o operador de desreferência, necessário para alterar o valor
}
```

### HANDS-ON

Vamos simular a análise de uma lista de valores de expressão gênica. Criaremos um vetor de números, calcularemos a média e depois "normalizaremos" os valores.

**1. Crie o projeto do Dia 13:**

```bash
cargo new dia13_vetores
cd dia13_vetores
```

**2. Edite o arquivo `src/main.rs`:**

```rust
// src/main.rs

fn main() {
    // 1. Criamos um vetor para armazenar os níveis de expressão de vários genes.
    // Usamos a macro 'vec!' para inicializá-lo.
    let mut niveis_expressao = vec![150.5, 200.0, 50.2, 95.8, 310.0];

    println!("--- Análise de Expressão Gênica ---");
    println!("Valores de expressão originais: {:?}", niveis_expressao);

    // 2. Chamamos uma função que lê os valores para calcular a média.
    // Passamos uma referência imutável (&).
    let media = calcular_media(&niveis_expressao);
    println!("A média de expressão é: {:.2}", media);

    // 3. Chamamos uma função que MODIFICA os valores para normalizá-los.
    // Passamos uma referência mutável (&mut).
    normalizar_valores(&mut niveis_expressao);
    println!("Valores de expressão normalizados: {:?}", niveis_expressao);

    // 4. Acessando um valor de forma segura com .get()
    match niveis_expressao.get(2) {
        Some(valor) => println!("O terceiro valor agora é: {:.2}", valor),
        None => println!("Não foi possível encontrar o terceiro valor."),
    }
}

/// Calcula a média de um vetor de f64.
/// Recebe um empréstimo IMUTÁVEL, pois só precisa ler os dados.
fn calcular_media(valores: &Vec<f64>) -> f64 {
    let mut soma = 0.0;
    // Iteramos sobre cada valor para somá-los.
    for valor in valores { // 'valores' aqui já é uma referência
        soma += valor;
    }
    soma / valores.len() as f64 // Retornamos a média
}


/// Divide cada valor no vetor por 10.0 para "normalizá-lo".
/// Recebe um empréstimo MUTÁVEL, pois precisa alterar os dados.
fn normalizar_valores(valores: &mut Vec<f64>) {
    // Iteramos sobre cada valor com uma referência mutável.
    for valor in valores {
        // Usamos o operador de desreferência '*' para alterar o valor para o qual a referência aponta.
        *valor = *valor / 10.0;
    }
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Análise de Expressão Gênica ---
Valores de expressão originais: [150.5, 200.0, 50.2, 95.8, 310.0]
A média de expressão é: 161.30
Valores de expressão normalizados: [15.05, 20.0, 5.02, 9.58, 31.0]
O terceiro valor agora é: 5.02
```

### Próximos Passos

No **Dia 14**, faremos uma **Revisão do Módulo 2**, combinando todos esses elementos em um projeto mais elaborado para solidificar nosso conhecimento.
