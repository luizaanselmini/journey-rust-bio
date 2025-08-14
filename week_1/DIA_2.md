# Jornada Rust 🦀 em Bioinformática - DIA 2

## As Moléculas do Código: Variáveis e Tipos Primitivos

No [Dia 1](https://github.com/mlfalco-bioinfo/journey-rust-bio/blob/main/week_1/DIA_01.md), preparamos nosso ambiente de desenvolvimento. Agora que nosso laboratório está montado, é hora de aprender a manusear os reagentes mais básicos: os dados. Neste segundo dia da jornada, vamos explorar como o Rust armazena informações usando variáveis e os tipos de dados fundamentais, sempre com um olhar para a nossa área, a bioinformática.

---

### Objetivos do Dia

1.  **Entender** como declarar variáveis com a palavra-chave `let`.
2.  **Aprender** o conceito de imutabilidade por padrão e como usar `mut` para criar variáveis mutáveis.
3.  **Conhecer** os tipos de dados primitivos (inteiros, floats, booleanos, caracteres).
4.  **Aplicar** esses conceitos para representar dados biológicos simples.

---

### Conceitos do Dia

#### Variáveis e Imutabilidade

Em Rust, declaramos uma variável com a palavra-chave `let`.

```rust
let numero_de_cromossomos = 46;
```

Um dos conceitos mais importantes de Rust é a **imutabilidade por padrão**. Isso significa que, uma vez que atribuímos um valor a uma variável, não podemos mudá-lo. Pense nisso como uma amostra de DNA que, uma vez preparada e etiquetada, não deve ser alterada para garantir a integridade do experimento. Isso torna o código mais seguro e fácil de entender.

Se tentássemos mudar o valor, o compilador nos daria um erro:
```rust
let numero_de_cromossomos = 46;
numero_de_cromossomos = 47; // ERRO DE COMPILAÇÃO!
```

#### Mutabilidade

Claro, às vezes precisamos de variáveis que mudam de valor (como um contador em um loop). Para isso, usamos a palavra-chave `mut` na declaração:

```rust
let mut contagem_de_reads = 0;
contagem_de_reads = contagem_de_reads + 1; // Funciona!
```

#### Tipos de Dados Primitivos

Rust é uma linguagem com tipagem estática, o que significa que toda variável precisa ter um tipo definido. Felizmente, o compilador é inteligente e consegue inferir o tipo na maioria das vezes. Os tipos básicos que mais usaremos são:

* **Inteiros:** Números sem casas decimais. Podem ser `i32` (inteiro de 32 bits, com sinal, pode ser negativo) ou `u32` (inteiro de 32 bits, sem sinal, apenas positivo).
    * Exemplo para bioinfo: `let score_alinhamento: i32 = -25;` (pode ser negativo)
    * Exemplo para bioinfo: `let comprimento_gene: u32 = 1500;` (não pode ser negativo)

* **Ponto Flutuante (Float):** Números com casas decimais. O padrão é `f64` (precisão dupla de 64 bits).
    * Exemplo para bioinfo: `let conteudo_gc: f64 = 0.425;`

* **Booleanos (`bool`):** Representa verdade ou falsidade, com os valores `true` ou `false`.
    * Exemplo para bioinfo: `let gene_codificante: bool = true;`

* **Caracteres (`char`):** Representa um único caractere Unicode, denotado por aspas simples.
    * Exemplo para bioinfo: `let primeira_base: char = 'A';`

---

### HANDS-ON

Vamos colocar a teoria em prática. Crie um novo projeto com o Cargo e adicione o código para ver as variáveis em ação.

**1. Crie o projeto do Dia 2:**

```bash
# No terminal, fora da pasta do projeto anterior
cargo new dia2_variaveis
cd dia2_variaveis
```

**2. Edite o arquivo `src/main.rs`:**

Abra o arquivo e substitua o conteúdo pelo código abaixo. Ele declara diversas variáveis representando dados biológicos e as imprime na tela.

```rust
// src/main.rs

fn main() {
    // --- Números Inteiros ---
    // Usamos 'u32' para um número que não pode ser negativo, como o número de cromossomos.
    let numero_cromossomos_humanos: u32 = 46;

    // Usamos 'i32' para um score que pode ser negativo.
    let score_alinhamento: i32 = -15;

    // --- Ponto Flutuante ---
    // 'f64' é o padrão para números decimais, ideal para frequências e probabilidades.
    let conteudo_gc: f64 = 0.62;
    let p_value: f64 = 0.049;

    // --- Booleanos ---
    // 'bool' para representar estados de verdadeiro/falso.
    let possui_codon_de_parada: bool = true;

    // --- Caracteres ---
    // 'char' para representar uma única base de DNA.
    let base_inicial: char = 'A';

    // --- Variável Mutável ---
    // Um contador para o número de genes encontrados, precisa ser mutável.
    let mut genes_encontrados = 0;
    genes_encontrados = genes_encontrados + 1; // Simulando a descoberta de um gene.

    // Imprimindo todos os nossos dados biológicos na tela!
    println!("--- Dados Biológicos em Rust ---");
    println!("Número de cromossomos em humanos: {}", numero_cromossomos_humanos);
    println!("Score de um alinhamento: {}", score_alinhamento);
    println!("Conteúdo GC da sequência: {}", conteudo_gc);
    println!("Valor de P do teste estatístico: {}", p_value);
    println!("A sequência possui códon de parada? {}", possui_codon_de_parada);
    println!("A primeira base da sequência é: {}", base_inicial);
    println!("Total de genes encontrados até agora: {}", genes_encontrados);
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Dados Biológicos em Rust ---
Número de cromossomos em humanos: 46
Score de um alinhamento: -15
Conteúdo GC da sequência: 0.62
Valor de P do teste estatístico: 0.049
A sequência possui códon de parada? true
A primeira base da sequência é: A
Total de genes encontrados até agora: 1
```
### Comentários

Os caracteres usados para comentários no Rust são `\\` diferentemente da maiorias das linguagens que usam o `#` e parecido com Java(JavaScritp) e C/C++.
Além do comentário único, existem comentários múltiplos (aninhamento que é um diferencial entre as linguagens de programação) e comentários direcionados para 
documentação.

---

### Próximos Passos

No **Dia 3**, vamos criar as "enzimas" do nosso código: as **Funções**, que permitirão operar os dados de forma organizada e reutilizável.
