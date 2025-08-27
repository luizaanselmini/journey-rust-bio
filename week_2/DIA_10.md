# Jornada Rust 🦀 em Bioinformática - DIA 10

## Structs: Modelando os Dados da Vida

Até agora em nossa jornada, trabalhamos com os "aminoácidos" da programação: tipos de dados simples como inteiros (`i32`), texto (`String`) e booleanos (`bool`). Mas a biologia é complexa. Um gene não é apenas um nome; ele tem um cromossomo, uma posição de início, uma posição de fim e uma sequência.

Como podemos agrupar todas essas informações relacionadas em uma única unidade lógica? A resposta em Rust são as **Structs**.

Uma `struct` (abreviação de *structure*, ou estrutura) é um tipo de dado customizado que nos permite nomear e agrupar múltiplos valores. Hoje, vamos aprender a construir nossas próprias "proteínas" de dados, criando `struct`s que modelam entidades biológicas do mundo real.

---

### Objetivos do Dia

1.  **Aprender** a sintaxe para definir uma `struct`.
2.  **Entender** como criar uma instância (um valor) de uma `struct`.
3.  **Aprender** a acessar e usar os dados dentro de uma `struct`.
4.  **Ver** como a posse (ownership) se aplica a `struct`s.
5.  **Conhecer** brevemente as *Tuple Structs*.

---

### Conceitos do Dia 

#### Definindo uma `Struct`

Para criar uma `struct`, usamos a palavra-chave `struct` e definimos um nome e um conjunto de campos (fields), cada um com seu próprio tipo.

```rust
// Definimos o "molde" para o que é um Gene.
struct Gene {
    nome: String,
    cromossomo: u8, // u8 para números de 1 a 22 + X/Y
    inicio: u32,
    fim: u32,
}
```
Com isso, criamos um novo tipo `Gene` em nosso programa, tão válido quanto `i32` ou `String`.

#### Criando uma Instância

Para usar uma `struct`, criamos uma **instância** dela, fornecendo valores concretos para cada campo.

```rust
let gene_tp53 = Gene {
    nome: String::from("TP53"),
    cromossomo: 17,
    inicio: 7_661_779, // O '_' pode ser usado para separar números e facilitar a leitura
    fim: 7_687_550,
};
```

#### Acessando os Dados

Usamos a **notação de ponto (`.`)** para acessar os valores dos campos de uma instância.

```rust
println!("Analisando o gene: {}", gene_tp53.nome);
let tamanho_gene = gene_tp53.fim - gene_tp53.inicio;
println!("Tamanho do gene: {} pares de base", tamanho_gene);
```

#### Posse (Ownership) e Structs

As regras de posse se aplicam à `struct` como um todo.
* No nosso exemplo, o campo `nome` é do tipo `String`, que é um tipo com posse.
* Se você atribuir uma instância de `Gene` a outra variável (`let outro_gene = gene_tp53;`), a **posse inteira** da `struct` (incluindo sua `String` interna) será movida. A `gene_tp53` original se tornará inválida.
* Para passar uma `struct` para uma função sem mover a posse, você usa uma referência (`&Gene`), assim como fizemos com `String`.

#### Tuple Structs

Existe uma variação chamada *tuple struct*, que é útil quando os nomes dos campos não são importantes.

```rust
// Uma cor RGB é apenas um conjunto de 3 números.
struct Cor(u8, u8, u8);
let cor_vermelha = Cor(255, 0, 0);
// Acessamos por índice:
let componente_vermelho = cor_vermelha.0;
```

---

### HANDS-ON

Vamos modelar um registro de um arquivo FASTA, que contém um ID e uma sequência, e depois criar uma função para analisá-lo.

**1. Crie o projeto do Dia 10:**

```bash
cargo new dia10_structs
cd dia10_structs
```

**2. Edite o arquivo `src/main.rs`:**

```rust
// src/main.rs

// 1. Definimos o molde para nossos dados.
// Queremos agrupar um ID e uma sequência de DNA.
struct RegistroFasta {
    id: String,
    sequencia: String,
}

fn main() {
    // 2. Criamos uma instância da nossa struct.
    let registro1 = RegistroFasta {
        id: String::from("gene_COX1_humano"),
        sequencia: String::from("GATTACATGGTT"),
    };

    // Criamos outra instância para um organismo diferente.
    let registro2 = RegistroFasta {
        id: String::from("gene_COX1_camundongo"),
        sequencia: String::from("GCTTACATGGCT"),
    };
    
    println!("--- Análise de Registros FASTA ---");

    // 3. Chamamos uma função para analisar nosso primeiro registro.
    // Passamos uma referência (&) para não mover a posse!
    imprimir_resumo_registro(&registro1);
    imprimir_resumo_registro(&registro2);
    
    // registro1 e registro2 continuam válidos aqui porque só emprestamos o acesso.
    println!("\nAnálise concluída. Ambos os registros ainda são de nossa posse.");
}


/// Uma função que recebe um EMPRÉSTIMO de um RegistroFasta e imprime um resumo.
/// O tipo do parâmetro é &RegistroFasta - uma referência à nossa struct.
fn imprimir_resumo_registro(registro: &RegistroFasta) {
    println!("\nAnalisando o registro com ID: {}", registro.id); // Acessamos os campos com '.'
    println!("  - Comprimento da sequência: {} pb", registro.sequencia.len());
    
    if registro.sequencia.starts_with("GATTA") {
        println!("  - A sequência começa com o motivo esperado.");
    } else {
        println!("  - A sequência NÃO começa com o motivo esperado.");
    }
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Análise de Registros FASTA ---

Analisando o registro com ID: gene_COX1_humano
  - Comprimento da sequência: 12 pb
  - A sequência começa com o motivo esperado.

Analisando o registro com ID: gene_COX1_camundongo
  - Comprimento da sequência: 12 pb
  - A sequência NÃO começa com o motivo esperado.

Análise concluída. Ambos os registros ainda são de nossa posse.
```

---


### Próximos Passos
No **Dia 11**, vamos explorar os **Enums**, a ferramenta perfeita para modelar estados e variações.
