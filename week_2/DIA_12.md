# Jornada Rust 🦀 em Bioinformática - DIA 12

## Lidando com a Incerteza: Erros e o Enum `Result`

Em nossa jornada até agora, a maioria dos nossos exemplos partiu do princípio de que tudo funciona perfeitamente. Mas na bioinformática, como na ciência, as coisas raramente são perfeitas. E se um arquivo de sequência estiver corrompido? E se tentarmos analisar uma sequência que contém caracteres inválidos? Um bom programa, assim como um bom protocolo de laboratório, precisa antecipar e saber como reagir a possíveis falhas.

Em muitas linguagens, os erros são tratados com exceções ou valores nulos, fontes comuns de bugs e falhas inesperadas. O Rust adota uma abordagem diferente e muito mais segura. No Dia 11, vimos o `enum Option` para lidar com a *ausência de um valor*. Hoje, vamos conhecer seu primo mais poderoso, o `enum Result`, que é a principal ferramenta do Rust para lidar com operações que podem **falhar**.

---

### Objetivos do Dia

1.  **Diferenciar** entre erros recuperáveis e irrecuperáveis (`panic!`).
2.  **Revisar** o `enum Option<T>` como a forma de lidar com a ausência de valores.
3.  **Aprender** a usar o `enum Result<T, E>` para lidar com operações que podem falhar.
4.  **Utilizar** a expressão `match` para manusear um `Result` de forma segura.
5.  **Conhecer** atalhos como `.unwrap()` e `.expect()` (e seus perigos).

---

### Conceitos do Dia

#### Erros Recuperáveis vs. Irrecuperáveis

Rust agrupa os erros em duas categorias:
* **Irrecuperáveis:** Erros que indicam um bug no programa e que não fazem sentido continuar, como tentar acessar um índice de array fora dos limites. A ação padrão é encerrar o programa imediatamente com a macro `panic!`.
* **Recuperáveis:** Erros que são esperados e que o programa deve saber como lidar, como um arquivo não encontrado ou dados em um formato inválido. Para estes, usamos o `enum Result`.

#### O `enum Result<T, E>`

Este `enum` é o pilar do tratamento de erros em Rust. Ele é definido da seguinte forma:

```rust
enum Result<T, E> {
    Ok(T),  // Contém o valor de sucesso, do tipo T.
    Err(E), // Contém o valor de erro, do tipo E.
}
```

Uma função que pode falhar, em vez de retornar apenas o valor de sucesso, retorna um `Result`. Isso força quem chamou a função a verificar se a operação deu certo (`Ok`) ou errado (`Err`) e a lidar com ambas as situações.

#### Manuseando `Result` com `match`

Assim como com `Option`, `match` é a forma mais explícita e segura de lidar com um `Result`, pois o compilador te obriga a tratar os dois casos:

```rust
let resultado = minha_funcao_que_pode_falhar();

match resultado {
    Ok(valor_de_sucesso) => {
        println!("A operação foi um sucesso! O valor é: {}", valor_de_sucesso);
    },
    Err(mensagem_de_erro) => {
        println!("A operação falhou! Erro: {}", mensagem_de_erro);
    },
}
```

#### Atalhos (Para Usar com Cuidado)

Às vezes, especialmente em exemplos rápidos ou testes, você tem certeza de que uma operação não vai falhar. Rust oferece alguns "atalhos" para extrair o valor de um `Result`, mas eles causarão um `panic!` se o resultado for um `Err`.

* `.unwrap()`: Se for `Ok`, retorna o valor. Se for `Err`, entra em `panic!`.
* `.expect("Mensagem de erro")`: Igual ao `unwrap()`, mas permite que você forneça uma mensagem de erro customizada para o `panic!`.

Esses métodos são úteis, mas em código de produção, prefira sempre o `match` ou o operador `?` (que veremos em dias futuros) para um tratamento de erros mais robusto.

---

### HANDS-ON

Vamos criar uma função que tenta "parsear" (analisar e converter) uma string para uma sequência de aminoácidos. A função pode falhar se encontrar um caractere inválido.

**1. Crie o projeto do Dia 12:**

```bash
cargo new dia12_erros
cd dia12_erros
```

**2. Edite o arquivo `src/main.rs`:**

```rust
// src/main.rs

// Enum para representar os aminoácidos que nossa função reconhece.
#[derive(Debug)] // Permite que a gente imprima com {:?}
enum Aminoacido {
    Alanina, // 'A'
    Glicina, // 'G'
}

/// Tenta converter uma string em um vetor de Aminoácidos.
/// Retorna Ok(Vetor) em caso de sucesso.
/// Retorna Err(&str) em caso de falha.
fn parse_sequencia_proteica(seq: &str) -> Result<Vec<Aminoacido>, &'static str> {
    let mut proteina = Vec::new(); // Vec é um vetor, uma lista que veremos em breve.

    for base in seq.chars() {
        match base {
            'A' => proteina.push(Aminoacido::Alanina),
            'G' => proteina.push(Aminoacido::Glicina),
            // Se encontrarmos um caractere que não conhecemos, a operação falha!
            _ => return Err("Caractere inválido encontrado na sequência!"),
        }
    }

    // Se o loop terminar sem erros, a operação foi um sucesso!
    Ok(proteina)
}


fn main() {
    let seq_valida = "AGGA";
    let seq_invalida = "AGXGA"; // Contém um 'X' inválido.

    println!("--- Testando com sequência válida: {} ---", seq_valida);
    let resultado1 = parse_sequencia_proteica(seq_valida);

    // Usamos 'match' para lidar com o Result retornado.
    match resultado1 {
        Ok(proteina) => println!("Sucesso! Sequência parseada: {:?}", proteina),
        Err(erro) => println!("Erro: {}", erro),
    }

    println!("\n--- Testando com sequência inválida: {} ---", seq_invalida);
    let resultado2 = parse_sequencia_proteica(seq_invalida);

    match resultado2 {
        Ok(proteina) => println!("Sucesso! Sequência parseada: {:?}", proteina),
        Err(erro) => println!("Erro: {}", erro),
    }
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Testando com sequência válida: AGGA ---
Sucesso! Sequência parseada: [Alanina, Glicina, Glicina, Alanina]

--- Testando com sequência inválida: AGXGA ---
Erro: Caractere inválido encontrado na sequência!
```

---

### Próximos Passos

No **Dia 13**, vamos explorar oficialmente os **Vetores**, a estrutura de dados mais comum para se trabalhar com coleções e listas de itens em Rust.
