# Jornada Rust 🦀 em Bioinformática - DIA 3

## Funções: As Enzimas do Nosso Código

Nos primeiros dois dias, preparamos nosso ambiente e aprendemos a armazenar dados em variáveis. Agora, precisamos de uma forma de operar sobre esses dados. Na biologia, as enzimas são proteínas que catalisam reações específicas de forma eficiente e reutilizável. Na programação, temos as **funções**.

Neste terceiro dia da jornada, vamos aprender a criar essas "enzimas" em nosso código: blocos de código nomeados que executam tarefas específicas, tornando nosso trabalho mais organizado, legível e poderoso.

---

### Objetivos do Dia

1.  **Aprender** a sintaxe para declarar e chamar funções em Rust.
2.  **Entender** como passar dados para funções através de parâmetros.
3.  **Descobrir** como retornar valores de uma função.
4.  **Diferenciar** entre declarações (statements) e expressões (expressions), um conceito chave em Rust.

---

### Conceitos do Dia

#### O que é uma Função?

Uma função é um bloco de código que encapsula uma tarefa específica. Você define a função uma vez e pode chamá-la (executá-la) quantas vezes quiser. A função principal da sua jornada até agora, a `fn main()`, é o ponto de entrada de todo programa executável em Rust.

#### Declarando Funções

Usamos a palavra-chave `fn`, seguida por um nome, parênteses `()` e chaves `{}`.

```rust
fn minha_funcao() {
    // código da função aqui
}
```

#### Parâmetros (As Entradas da Enzima)

Podemos passar informações para nossas funções através de parâmetros. Uma grande diferença em Rust é que você **deve** declarar o tipo de cada parâmetro.

```rust
// Esta função recebe um número de gene do tipo u32
fn imprimir_id_gene(id_gene: u32) {
    println!("O ID do gene é: {}", id_gene);
}
```

#### Valores de Retorno (Os Produtos da Reação)

As funções também podem retornar valores. Para isso, declaramos o tipo do retorno após uma seta `->`.

Em Rust, existem duas formas de retornar um valor:
1.  Usando a palavra-chave `return` explicitamente.
2.  **O jeito de Rust:** O valor da última *expressão* no corpo da função é retornado automaticamente.

**Importante:**
* **Declarações (Statements):** São instruções que realizam uma ação mas não retornam um valor. Elas terminam com ponto e vírgula `;`. Ex: `let x = 5;`.
* **Expressões (Expressions):** Avaliam para um valor resultante. Ex: `5 + 1`. Em Rust, `if`, `match` e até blocos `{}` são expressões. **Uma expressão de retorno não deve ter um ponto e vírgula no final!**

Veja a diferença:

```rust
fn soma_um(x: i32) -> i32 {
    x + 1 // Expressão: avalia para x + 1 e retorna o valor. SEM ponto e vírgula!
}

fn soma_dois(x: i32) -> i32 {
    let resultado = x + 2;
    return resultado; // Usando a palavra-chave 'return' explicitamente.
}
```

---

### HANDS-ON

Vamos criar uma função que simula a transcrição de uma única base de DNA para RNA.

**1. Crie o projeto do Dia 3:**

```bash
# No terminal, saia da pasta do dia anterior se necessário
cargo new dia3_funcoes
cd dia3_funcoes
```

**2. Edite o arquivo `src/main.rs`:**

Substitua o conteúdo pelo código abaixo. Ele define nossa função "enzimática" `transcrever_base` e a chama várias vezes a partir da `main`.

```rust
// src/main.rs

// Nossa função principal, o ponto de entrada do programa.
fn main() {
    let base1 = 'G';
    let base2 = 'C';
    let base3 = 'T';
    let base4 = 'A';

    // Chamamos nossa função para cada base e guardamos o resultado.
    let rna1 = transcrever_base(base1);
    let rna2 = transcrever_base(base2);
    let rna3 = transcrever_base(base3);
    let rna4 = transcrever_base(base4);

    println!("A transcrição de {} é {}", base1, rna1);
    println!("A transcrição de {} é {}", base2, rna2);
    println!("A transcrição de {} é {}", base3, rna3);
    println!("A transcrição de {} é {}", base4, rna4);
}

/// Esta função recebe uma base de DNA e retorna a base de RNA correspondente.
/// Parâmetro 'base_dna' é do tipo char.
/// O valor de retorno '->' também é do tipo char.
fn transcrever_base(base_dna: char) -> char {
    // Usamos a expressão 'match' para comparar a base de entrada.
    // 'match' é muito poderoso em Rust e o veremos em mais detalhes.
    match base_dna {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A', // Em uma transcrição real, T -> A
        'A' => 'U', // A -> U é a regra da transcrição para RNA
        // O '_' é um curinga para qualquer caso.
        _   => 'N', // Retorna 'N' para uma base inválida.
    }
    // Note que todo o bloco 'match' é uma EXPRESSÃO.
    // O valor do braço que for executado será o valor de retorno desta função.
    // Por isso, não há ponto e vírgula aqui.
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
A transcrição de G é C
A transcrição de C é G
A transcrição de T é A
A transcrição de A é U
```

---


### Próximos Passos

Agora que temos variáveis (moléculas) e funções (enzimas), como fazemos nosso código tomar decisões e seguir caminhos diferentes? 
No **Dia 4**, vamos explorar o **Controle de Fluxo** com `if/else`, as vias de sinalização do nosso código.
