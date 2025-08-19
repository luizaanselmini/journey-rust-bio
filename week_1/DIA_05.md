# Jornada Rust 🦀 em Bioinformática - DIA 5

## Strings: O DNA do Nosso Código

Até agora em nossa jornada, aprendemos a guardar dados, criar funções e controlar o fluxo do programa. Hoje, vamos nos aprofundar no tipo de dado que é a base da genômica e da bioinformática: o texto, ou como o chamamos em programação, **Strings**.

Seja uma sequência de DNA, um alinhamento de proteínas ou o ID de uma amostra, tudo é texto. Entender como Rust lida com strings de forma segura e eficiente é uma das habilidades mais importantes que desenvolveremos. A abordagem de Rust é poderosa, mas também possui detalhes importantes que a diferenciam de outras linguagens.

---

### Objetivos do Dia

1.  **Entender** a diferença fundamental entre `&str` (string slice) e `String`.
2.  **Aprender** a criar, modificar e concatenar `String`s.
3.  **Descobrir** a maneira segura de iterar sobre os caracteres de uma string.
4.  **Utilizar** a macro `format!` para construir strings de forma limpa.

---

### Conceitos do Dia

#### A Dupla Hélice das Strings: `&str` e `String`

Em Rust, existem duas formas principais de se trabalhar com texto, e entender a diferença é crucial:

1.  **`&str` (string slice):**
    * Pense nisso como uma **referência** ou uma **visão** imutável de uma sequência de texto que está armazenada em outro lugar.
    * É leve e rápido.
    * Literais de string, como `"ACGT"`, são do tipo `&str`.
    * **Analogia:** Um `&str` é como um link para uma página da web. Você pode ler o conteúdo, mas não pode alterar a página original através do link.

2.  **`String`:**
    * É um tipo de dado **proprietário**, mutável e que pode crescer, armazenado na memória *heap*.
    * Você usa `String` quando precisa criar ou modificar texto durante a execução do programa.
    * **Analogia:** Uma `String` é como um documento de texto no seu computador. Ele é seu, você pode abri-lo, adicionar texto, apagar e salvá-lo.

#### Criando e Modificando uma `String`

Geralmente, você começa com um `&str` e o converte para uma `String` quando precisa de flexibilidade.

```rust
// Criando uma String a partir de um string slice (&str)
let seq_dna = "ACGTG".to_string();
// Ou usando String::from()
let seq_rna = String::from("ACGUGC");

// Criando uma String vazia para construir aos poucos
let mut gene_id = String::new();

// Adicionando texto a uma String
gene_id.push_str("TP53"); // push_str recebe um &str (uma referência)
gene_id.push('-'); // push recebe um único char
gene_id.push_str("exon1");
```

#### Juntando Strings

* **Com o operador `+`:** Você pode usar `+` para concatenar, mas há um detalhe de posse (ownership).
    ```rust
    let s1 = String::from("Olá, ");
    let s2 = String::from("Mundo!");
    let s3 = s1 + &s2; // s1 é "movida" e não pode mais ser usada!
    ```

* **Com a macro `format!` (Recomendado):** Esta é a forma mais limpa, legível e flexível de combinar strings. Ela não toma posse das variáveis.
    ```rust
    let gene = "BRCA1";
    let status = "ativo";
    let relatorio = format!("Relatório do gene: {} - Status: {}", gene, status);
    ```

#### Acessando Caracteres

Devido ao suporte completo a Unicode (UTF-8), acessar um caractere por índice (como `seq[0]`) não é permitido diretamente em Rust, pois um "caractere" pode ocupar mais de um byte. A forma segura e correta de processar uma sequência é iterando sobre seus caracteres:

```rust
for base in seq_dna.chars() {
    println!("Base encontrada: {}", base);
}
```

---

### HANDS-ON

Vamos praticar a criação e manipulação de strings para montar um identificador de sequência.

**1. Crie o projeto do Dia 5:**

```bash
cargo new dia5_strings
cd dia5_strings
```

**2. Edite o arquivo `src/main.rs`:**

Substitua o conteúdo pelo código abaixo.

```rust
// src/main.rs

fn main() {
    // 1. Criar uma String a partir de um &str
    let mut sequencia_dna = String::from("ATGCGGTCAT");

    println!("--- Análise Inicial da Sequência ---");
    println!("Sequência original: {}", sequencia_dna);
    println!("Tamanho inicial: {} bases", sequencia_dna.len());

    // 2. Modificar a String, adicionando uma região desconhecida
    let regiao_desconhecida = "NNNNN";
    sequencia_dna.push_str(regiao_desconhecida);

    println!("\n--- Sequência Modificada ---");
    println!("Sequência após adição: {}", sequencia_dna);
    println!("Tamanho final: {} bases", sequencia_dna.len());

    // 3. Construir um identificador de sequência usando format!
    let nome_gene = String::from("COX1");
    let organismo = "Homo sapiens";
    let id_acesso = 12345;

    let identificador_completo = format!(
        ">{}:{} | Acesso #{}",
        nome_gene,
        organismo,
        id_acesso
    );

    println!("\n--- Identificador no Formato FASTA ---");
    println!("{}", identificador_completo);

    // 4. Iterar sobre a sequência original usando .chars()
    println!("\n--- Iterando sobre as bases da sequência '{}' ---", nome_gene);
    // Note que usamos a String 'nome_gene' aqui sem problemas, pois format! não a consumiu.
    for caractere in nome_gene.chars() {
        println!("Caractere: {}", caractere);
    }
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Análise Inicial da Sequência ---
Sequência original: ATGCGGTCAT
Tamanho inicial: 10 bases

--- Sequência Modificada ---
Sequência após adição: ATGCGGTCATNNNNN
Tamanho final: 15 bases

--- Identificador no Formato FASTA ---
>COX1:Homo sapiens | Acesso #12345

--- Iterando sobre as bases da sequência 'COX1' ---
Caractere: C
Caractere: O
Caractere: X
Caractere: 1
```

---

### Próximos Passos

Hoje, vimos os fundamentos de como o Rust lida com dados de texto, uma habilidade vital para nossa jornada. Entendemos a diferença entre `String` e `&str` e aprendemos a manipulá-los de forma segura.

No **Dia 6**, vamos aprender sobre **Loops**, as ferramentas que nos permitem executar blocos de código repetidamente.
