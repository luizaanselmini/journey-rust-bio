# Jornada Rust 🦀 em Bioinformática - DIA 9

## Empréstimos e Referências: Compartilhando sem Perder a Posse

No último dia da nossa jornada, aprendemos a regra mais importante do Rust: a **Posse (Ownership)**. Vimos que, ao passar dados complexos como uma `String`, a posse é transferida (`move`) e o dono original se torna inválido. Isso nos deixou com uma questão crucial: "Como posso deixar uma função usar meus dados sem que eu perca o acesso a eles?"

A resposta é o conceito de **Empréstimo (Borrowing)**. Em vez de entregar a posse permanentemente, podemos "emprestar" o acesso aos nossos dados por um tempo. Para fazer isso, usamos uma **Referência**.

Pense na analogia do equipamento de laboratório do Dia 8. Em vez de entregar a única chave da máquina (transferir a posse), você pode dar a um colega um **crachá de acesso temporário** (uma referência) que o permite usar a máquina. Quando ele termina, o crachá expira e você continua sendo o único dono da chave.

---

### Objetivos do Dia

1.  **Entender** o que é Empréstimo (Borrowing) e por que ele é necessário.
2.  **Aprender** a criar e usar referências imutáveis (`&T`).
3.  **Aprender** a criar e usar referências mutáveis (`&mut T`).
4.  **Conhecer** as regras do *Borrow Checker* para garantir a segurança dos dados.

---

### Conceitos do Dia

#### Referências Imutáveis (`&`)

Uma referência é como um ponteiro que aponta para um endereço de memória, mas com garantias de segurança. Criamos uma referência usando o operador `&` (e-comercial). Por padrão, as referências são imutáveis, o que significa que você pode ler os dados, mas não pode modificá-los.

```rust
fn main() {
    let s1 = String::from("GATTACA");
    
    // Passamos uma REFERÊNCIA (&s1) para a função, não a posse.
    let tamanho = calcular_tamanho(&s1); 

    // s1 continua válida aqui, pois a posse nunca foi movida!
    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

// A função recebe um &String, uma referência a uma String.
fn calcular_tamanho(s: &String) -> usize {
    s.len() // Podemos ler o dado através da referência.
} // A referência 's' sai de escopo aqui, e nada acontece com a s1 original.
```

#### Referências Mutáveis (`&mut`)

E se quisermos que uma função modifique nossos dados? Para isso, criamos uma **referência mutável** usando `&mut`.

```rust
fn main() {
    // A variável precisa ser 'mut' para que possamos emprestá-la como mutável.
    let mut sequencia = String::from("ATGC"); 
    
    // Passamos uma referência MUTÁVEL.
    adicionar_codon_de_parada(&mut sequencia);

    println!("A sequência final é: {}", sequencia);
}

// A função aceita uma referência MUTÁVEL a uma String.
fn adicionar_codon_de_parada(s: &mut String) {
    s.push_str("TAA"); // Podemos modificar a String original através da referência.
}
```

#### As Regras do Empréstimo (A Fiscalização do Compilador)

Para garantir a segurança total e evitar erros como "corridas de dados" (*data races*), o compilador do Rust (o *Borrow Checker*) impõe regras rígidas sobre os empréstimos:

1.  Em um determinado escopo, você pode ter **ou**
    * **Uma** referência mutável (`&mut T`).
    * **Qualquer número** de referências imutáveis (`&T`).
2.  Você **não pode** ter uma referência mutável e referências imutáveis ao mesmo tempo.
3.  As referências devem ser sempre válidas (apontar para dados que existem).

**Analogia:** Pense em um documento no Google Docs.
* **Referências Imutáveis:** Várias pessoas podem ter o link com permissão de "Leitor". Todas podem olhar o documento ao mesmo tempo.
* **Referência Mutável:** Apenas **uma** pessoa por vez pode ter a permissão de "Editor". Enquanto alguém está editando, ninguém mais (nem mesmo leitores) pode acessar para evitar conflitos.

---

### HANDS-ON

Vamos aplicar os conceitos de empréstimo imutável e mutável em um pequeno programa de análise.

**1. Crie o projeto do Dia 9:**

```bash
cargo new dia9_borrowing
cd dia9_borrowing
```

**2. Edite o arquivo `src/main.rs`:**

```rust
// src/main.rs

fn main() {
    // 1. Criamos a dona da String. A variável é 'mut' pois pretendemos modificá-la depois.
    let mut seq = String::from("GATTACAT");

    println!("Sequência original: {}", seq);

    // 2. Empréstimo Imutável (&)
    // Passamos um "crachá de leitor" para a função 'inspecionar_sequencia'.
    inspecionar_sequencia(&seq);

    // A variável 'seq' ainda é a dona e está 100% válida aqui.
    println!("Após a inspeção, a sequência continua sendo: {}", seq);

    // 3. Empréstimo Mutável (&mut)
    // Agora passamos um "crachá de editor" para a função 'corrigir_sequencia'.
    corrigir_sequencia(&mut seq);

    // A variável 'seq' ainda é a dona, mas seu conteúdo foi alterado pela função.
    println!("Após a correção, a sequência agora é: {}", seq);
}

/// Esta função pega emprestado o acesso de LEITURA a uma String.
fn inspecionar_sequencia(s: &String) {
    println!("  -> [Inspeção] O tamanho da sequência é {}.", s.len());
    if s.starts_with("GATTA") {
        println!("  -> [Inspeção] A sequência parece válida.");
    }
    // Não podemos fazer: s.push_str("A"); // ERRO! 's' é uma referência imutável.
}

/// Esta função pega emprestado o acesso de ESCRITA a uma String.
fn corrigir_sequencia(s: &mut String) {
    println!("  -> [Correção] A sequência está sendo modificada...");
    s.push_str("G"); // Adicionamos a base que faltava no final.
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
Sequência original: GATTACAT
  -> [Inspeção] O tamanho da sequência é 8.
  -> [Inspeção] A sequência parece válida.
Após a inspeção, a sequência continua sendo: GATTACAT
  -> [Correção] A sequência está sendo modificada...
Após a correção, a sequência agora é: GATTACATG
```

---

### Próximos Passos

No **Dia 10**, vamos aprender a usar as **Structs** para modelar nossas próprias entidades biológicas, como um Gene, um Cromossomo ou um Registro FASTA.
