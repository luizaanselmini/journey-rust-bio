# Jornada Rust 🦀 em Bioinformática - DIA 6

## Loops: Automatizando a Análise com a PCR do Código

Em nossa jornada até agora, aprendemos a guardar, manipular e tomar decisões com base em dados. No Dia 5, focamos em como lidar com strings, o "DNA" do nosso código. Mas imagine que você precisa encontrar todas as ocorrências da base 'G' em um gene de 3 bilhões de pares de base. Fazer isso manualmente seria impossível.

Na biologia molecular, técnicas como a PCR (Reação em Cadeia da Polimerase) nos permitem amplificar e trabalhar com milhões de cópias de DNA de forma automatizada. Em programação, a ferramenta análoga para a automação de tarefas repetitivas são os **loops**.

Neste sexto dia, vamos aprender a construir esses "ciclos de amplificação" em Rust para iterar sobre dados e executar tarefas repetidamente, economizando tempo e tornando nosso código muito mais poderoso.

---

### Objetivos do Dia

1.  **Conhecer** os três tipos de loops em Rust: `loop`, `while` e `for`.
2.  **Aprender** a usar `break` para sair de um loop e `continue` para pular para a próxima iteração.
3.  **Entender** por que o loop `for` é a ferramenta mais comum e segura para iterar sobre coleções de dados em Rust.
4.  **Aplicar** um loop `for` para analisar uma sequência de DNA.

---

### Conceitos do Dia

Rust oferece três abordagens para tarefas repetitivas:

#### 1. `loop`: O Loop Infinito

O `loop` é a forma mais simples. Ele cria um ciclo que se repetirá para sempre, a menos que você o interrompa explicitamente com a palavra-chave `break`.

```rust
let mut contador = 0;
loop {
    println!("Repetindo...");
    contador += 1;
    if contador == 5 {
        break; // Para o loop quando o contador chegar a 5
    }
}
```
Um `loop` também pode retornar um valor após o `break`, o que pode ser útil.

#### 2. `while`: O Loop Condicional

Um loop `while` executa um bloco de código enquanto uma condição for verdadeira (`true`). Ele testa a condição *antes* de cada iteração.

```rust
let mut numero = 5;
while numero != 0 {
    println!("{}!", numero);
    numero -= 1;
}
println!("Lançar!");
```

#### 3. `for`: O Iterador

Este é, de longe, o tipo de loop mais utilizado e seguro em Rust. O loop `for` é excelente para executar um bloco de código para cada item de uma coleção, como um vetor ou os caracteres de uma string.

Ele é mais seguro que o `while` para percorrer coleções, pois o compilador garante que você não acesse um índice inválido, evitando erros de "off-by-one" (errar por um).

```rust
let sequencia = "ACGT";

// O método .chars() cria um iterador sobre os caracteres da string.
for base in sequencia.chars() {
    println!("A base é: {}", base);
}

// Iterando sobre um intervalo de números
for numero in 1..=5 { // O intervalo '1..=5' inclui o 1, 2, 3, 4 e 5.
    println!("O número é: {}", numero);
}
```

**Importante** A palavra-chave `continue` pode ser usada dentro de qualquer loop para parar a iteração atual e pular imediatamente para a próxima.

---

### HANDS-ON

Vamos usar o poderoso loop `for` para uma tarefa clássica de bioinformática: contar a ocorrência de uma base específica em uma sequência de DNA.

**1. Crie o projeto do Dia 6:**

```bash
cargo new dia6_loops
cd dia6_loops
```

**2. Edite o arquivo `src/main.rs`:**

Substitua o conteúdo pelo código abaixo. Vamos criar uma função que usa um `for` loop para essa tarefa.

```rust
// src/main.rs

fn main() {
    let sequencia_dna = "AGCTTGGAACATGCGATTACAG";
    let base_alvo = 'G';

    // Chamamos a função para contar as ocorrências da base 'G'.
    let contagem = contar_ocorrencias_base(sequencia_dna, base_alvo);

    println!("Analisando a sequência: {}", sequencia_dna);
    println!("O número de ocorrências da base '{}' é: {}", base_alvo, contagem);

    println!("\n--- Exemplo com 'continue' ---");
    imprimir_bases_nao_n(sequencia_dna);
}

/// Conta quantas vezes uma base específica aparece em uma sequência de DNA.
/// Usa um loop 'for' para iterar sobre cada caractere.
fn contar_ocorrencias_base(sequencia: &str, base_alvo: char) -> u32 {
    // Começamos com um contador mutável zerado.
    let mut contador: u32 = 0;

    // Iteramos sobre cada 'base' na 'sequencia'.
    // O método .chars() retorna um iterador, que o 'for' consome.
    for base in sequencia.chars() {
        // Se a base atual for igual à nossa base alvo...
        if base == base_alvo {
            // ...incrementamos o contador.
            contador += 1;
        }
    }

    // Retornamos o resultado final.
    contador
}

/// Imprime todas as bases de uma sequência, pulando as bases 'N' (desconhecidas).
fn imprimir_bases_nao_n(sequencia: &str) {
    println!("Imprimindo apenas bases conhecidas (não 'N') da sequência...");
    for base in sequencia.chars() {
        if base == 'N' {
            println!("  - Base 'N' encontrada, pulando para a próxima.");
            continue; // Pula o resto do código do loop e vai para a próxima iteração.
        }
        // Este println! só será executado se a base não for 'N'.
        println!("  - Processando base: {}", base);
    }
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
Analisando a sequência: AGCTTGGAACATGCGATTACAG
O número de ocorrências da base 'G' é: 6

--- Exemplo com 'continue' ---
Imprimindo apenas bases conhecidas (não 'N') da sequência...
  - Processando base: A
  - Processando base: G
  - Processando base: C
  - Processando base: T
  - Processando base: T
  - Processando base: G
  - Processando base: G
  - Processando base: A
  - Processando base: A
  - Processando base: C
  - Processando base: A
  - Processando base: T
  - Processando base: G
  - Processando base: C
  - Processando base: G
  - Processando base: A
  - Processando base: T
  - Processando base: T
  - Processando base: A
  - Processando base: C
  - Processando base: A
  - Processando base: G
```

---

### Próximos Passos

Até aqui vimos como configurar o ambiente, usar variáveis, funções, controle de fluxo, strings e loops. 

No **Dia 7**, faremos uma **revisão da semana**, consolidando com um pequeno projeto prático.
