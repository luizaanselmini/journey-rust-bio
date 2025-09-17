# Jornada Rust 🦀 em Bioinformática - DIA 4

## Controle de Fluxo: As Vias de Sinalização do Código

Nos dias anteriores, montamos nosso laboratório, aprendemos a guardar "moléculas" de dados em variáveis e a criar "enzimas" com funções para processá-los. Mas como uma célula que responde a diferentes sinais para ativar ou desativar genes, nosso código precisa de mecanismos para tomar decisões e executar ações diferentes com base nas condições do ambiente.

Neste quarto dia da jornada, vamos explorar as estruturas de controle de fluxo `if` e `else`. Elas são as "vias de sinalização" que permitem ao nosso programa analisar dados e decidir qual caminho seguir.

---

### Objetivos do Dia

1.  **Aprender** a sintaxe das estruturas condicionais `if`, `else if` e `else`.
2.  **Utilizar** operadores lógicos para criar condições complexas.
3.  **Descobrir** o poderoso conceito de `if` como uma expressão em Rust.
4.  **Aplicar** o controle de fluxo para classificar dados biológicos.

---

### Conceitos do Dia

#### Condições com `if`, `else if` e `else`

A estrutura condicional mais comum em programação é o bloco `if`. Ele permite que um trecho de código seja executado apenas se uma determinada condição for verdadeira (`true`).

```rust
let temperatura = 37.0;
if temperatura > 36.8 {
    println!("A amostra está com a temperatura um pouco elevada.");
}
```

Podemos adicionar um bloco `else` para executar um código alternativo caso a condição seja falsa (`false`). E para testar múltiplas condições em sequência, usamos `else if`.

```rust
let ph = 7.4;
if ph < 7.0 {
    println!("Solução ácida.");
} else if ph > 7.0 {
    println!("Solução básica.");
} else {
    println!("Solução neutra.");
}
```

**Importante** A identação não tem sentido sintático em Rust, a estrutura do código é formatada com as `{}`, em rust existe um formatador automático `rustfmt`, caso o código não esteja bem estruturado o formatador vai corrigir e deixar o código mais *legível*. Lembrando que, a comunidade Rust presa pela **legibilidade humana**, tanto que acrescentaram o formatador `rustfmt`.

Assim como em outras linguagens de programação, para construir as condições, usamos operadores lógicos como `==` (igual a), `!=` (diferente de), `<` (menor que), `>` (maior que), `&&` (E lógico) e `||` (OU lógico).

#### `if` é uma Expressão

Aqui está uma das características elegantes do Rust: `if` não é apenas uma declaração, é também uma **expressão**. Isso significa que ele "retorna" um valor, o que nos permite usá-lo para atribuir valores a uma variável de forma concisa.

```rust
let idade = 20;
let categoria = if idade < 18 { "Jovem" } else { "Adulto" };
println!("A categoria é: {}", categoria); // Imprime: A categoria é: Adulto
```

**Regra crucial:** O valor de retorno de todos os blocos (`if`, `else if`, `else`) **deve ser do mesmo tipo**. No exemplo acima, ambos os blocos retornam um `&str` (uma referência a uma string). O compilador do Rust garante essa consistência, prevenindo uma classe inteira de bugs!

---

### HANDS-ON

Vamos escrever um programa que usa o controle de fluxo para analisar algumas métricas biológicas, como o conteúdo GC e o tamanho de um gene.

**1. Crie o projeto do Dia 4:**

```bash
# No terminal, saia da pasta do dia anterior se necessário
cargo new dia4_controle_fluxo
cd dia4_controle_fluxo
```

**2. Edite o arquivo `src/main.rs`:**

Substitua o conteúdo pelo código abaixo. Ele contém duas funções: uma que usa `if/else` como declaração (para imprimir mensagens) e outra que o usa como expressão (para retornar um valor).

```rust
// src/main.rs

fn main() {
    let conteudo_gc1 = 0.35; // Conteúdo GC baixo
    let conteudo_gc2 = 0.55; // Conteúdo GC médio
    let conteudo_gc3 = 0.71; // Conteúdo GC alto

    println!("Analisando conteúdo GC:");
    analisar_conteudo_gc(conteudo_gc1);
    analisar_conteudo_gc(conteudo_gc2);
    analisar_conteudo_gc(conteudo_gc3);

    println!("\nAnalisando tamanho de genes:");
    let tamanho_gene1 = 150; // Pequeno
    let tamanho_gene2 = 2500; // Médio
    let tamanho_gene3 = 15000; // Grande

    // Chamamos a função e guardamos a classificação retornada.
    let classificacao1 = classificar_tamanho_gene(tamanho_gene1);
    let classificacao2 = classificar_tamanho_gene(tamanho_gene2);
    let classificacao3 = classificar_tamanho_gene(tamanho_gene3);

    println!("O gene de {} pares de base é considerado '{}'", tamanho_gene1, classificacao1);
    println!("O gene de {} pares de base é considerado '{}'", tamanho_gene2, classificacao2);
    println!("O gene de {} pares de base é considerado '{}'", tamanho_gene3, classificacao3);
}

/// Usa 'if/else' como declarações para imprimir uma análise.
/// Esta função não retorna um valor.
fn analisar_conteudo_gc(gc: f64) {
    if gc > 0.65 {
        println!("  - Conteúdo GC de {} é considerado ALTO.", gc);
    } else if gc > 0.45 {
        println!("  - Conteúdo GC de {} é considerado MÉDIO.", gc);
    } else {
        println!("  - Conteúdo GC de {} é considerado BAIXO.", gc);
    }
}

/// Usa 'if/else' como uma expressão para retornar um valor.
/// O valor de retorno é um '&'static str', uma referência a uma string que vive por toda a duração do programa.
fn classificar_tamanho_gene(comprimento: u32) -> &'static str {
    // A variável 'classificacao' recebe o valor da expressão 'if'.
    let classificacao = if comprimento > 10000 {
        "Grande"
    } else if comprimento > 2000 {
        "Médio"
    } else {
        "Pequeno"
    };
    // Retorna o valor que foi atribuído.
    classificacao
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
Analisando conteúdo GC:
  - Conteúdo GC de 0.35 é considerado BAIXO.
  - Conteúdo GC de 0.55 é considerado MÉDIO.
  - Conteúdo GC de 0.71 é considerado ALTO.

Analisando tamanho de genes:
O gene de 150 pares de base é considerado 'Pequeno'
O gene de 2500 pares de base é considerado 'Médio'
O gene de 15000 pares de base é considerado 'Grande'
```

---

### Próximos Passos

Hoje, demos inteligência ao nosso código, permitindo que ele reaja a diferentes dados de entrada. Vimos como `if/else` funciona e como o fato de ser uma expressão em Rust nos ajuda a escrever código mais limpo e seguro.
No **Dia 5**, vamos mergulhar em um dos tipos de dados mais importantes para a bioinformática, as **Strings**. Aprenderemos a manipulá-las de forma eficiente e segura, um passo essencial para qualquer análise de sequências.
