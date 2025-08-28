# Jornada Rust 🦀 em Bioinformática - DIA 11

## Enums: Modelando Variações e Estados Biológicos

Em nossa jornada, no Dia 10, aprendemos a usar `struct`s para agrupar dados diferentes em uma única unidade, como um "gene" que tem um nome, cromossomo e posição. Mas e se um dado puder ser uma de *várias coisas possíveis*? Por exemplo, uma base de DNA só pode ser A, C, G ou T. Um gene pode estar no sentido *forward* ou *reverse*. Como podemos representar esse tipo de escolha em nosso sistema de tipos?

A resposta em Rust são as **Enums** (enumerações). Um `enum` é um tipo customizado que pode ser um de seus vários "variantes" possíveis.

Hoje, vamos aprender a usar `enum`s para tornar nosso código mais seguro e expressivo, garantindo que nossos dados biológicos só possam conter valores válidos, eliminando toda uma classe de bugs em tempo de compilação.

---

### Objetivos do Dia

1.  **Aprender** a sintaxe para definir e usar um `enum`.
2.  **Entender** como o `match` é a ferramenta perfeita para trabalhar com `enum`s.
3.  **Descobrir** como as `enum`s podem garantir a validade dos dados em tempo de compilação.
4.  **Conhecer** o `Option`, um `enum` onipresente na biblioteca padrão do Rust.

---

### Conceitos do Dia 

#### Definindo um `Enum`

Um `enum` é definido com a palavra-chave `enum`, um nome, e uma lista de variantes dentro de chaves.

```rust
// Definimos um novo tipo chamado BaseDna.
// Uma variável do tipo BaseDna SÓ PODE ser um destes quatro valores.
enum BaseDna {
    A,
    C,
    G,
    T,
}
```
Isso é muito mais seguro do que usar um `char`, pois um `char` poderia ser 'X', 'Z' ou '5'. Com o `enum`, o compilador garante que apenas bases válidas sejam representadas.

#### Usando `Enum`s com `match`

A verdadeira força dos `enum`s aparece quando os combinamos com a expressão `match`. O `match` em Rust é como um `switch` superpoderoso. Ele permite que você compare um valor com uma série de padrões e execute código com base no padrão que corresponde.

O mais importante é que o `match` é **exaustivo**: você **deve** cobrir todas as variantes possíveis do `enum`. Se você esquecer uma, o compilador dará um erro, te forçando a lidar com todos os casos e prevenindo bugs.

```rust
fn obter_complemento(base: BaseDna) -> BaseDna {
    match base {
        BaseDna::A => BaseDna::T,
        BaseDna::C => BaseDna::G,
        BaseDna::G => BaseDna::C,
        BaseDna::T => BaseDna::A,
    }
    // Se esquecêssemos um dos braços, o código não compilaria!
}
```

#### O `Enum` `Option`

Você já se perguntou como Rust lida com valores nulos ou ausentes? A resposta é o `enum` `Option<T>`, que é tão importante que está na biblioteca padrão. Ele é definido assim:

```rust
enum Option<T> {
    None,       // Representa a ausência de um valor.
    Some(T),    // Representa a presença de um valor do tipo T.
}
```
Quando uma função pode ou não retornar um valor, em vez de retornar `null` (o que causa inúmeros erros em outras linguagens), ela retorna um `Option`. Por exemplo, uma função que busca um gene em um genoma pode retornar `Some(gene)` se o encontrar, ou `None` se não encontrar. O uso do `match` te força a lidar com o caso `None`, tornando seu código muito mais robusto.

---

### HANDS-ON

Vamos refinar nosso modelo de `Gene` usando `enum`s e `struct`s juntas. Vamos criar um `enum` para o sentido da transcrição e outro para as bases de DNA, e depois usá-los em uma `struct`.

**1. Crie o projeto do Dia 11:**

```bash
cargo new dia11_enums
cd dia11_enums
```

**2. Edite o arquivo `src/main.rs`:**

```rust
// src/main.rs

// 1. Definimos nossos tipos customizados com enums.

// Um aminoácido pode ser um de vários tipos.
#[derive(Debug)] // O 'derive' nos permite imprimir o enum com {:?}
enum Aminoacido {
    Alanina,
    Glicina,
    Leucina,
    Serina,
    // ... poderíamos adicionar todos os 20
}

// O sentido da fita de DNA.
#[derive(Debug)]
enum SentidoFita {
    Forward, // 5' -> 3'
    Reverse, // 3' -> 5'
}

// 2. Usamos nosso enum em uma struct.
#[derive(Debug)]
struct Proteina {
    nome: String,
    sentido: SentidoFita,
    comprimento: u32,
}


fn main() {
    // 3. Criamos instâncias usando nossos enums.
    let proteina_a = Proteina {
        nome: String::from("Albumina"),
        sentido: SentidoFita::Forward,
        comprimento: 585,
    };
    
    let aminoacido_lido = Aminoacido::Leucina;

    println!("--- Análise de Proteínas e Aminoácidos ---");
    println!("Proteína encontrada: {:?}", proteina_a);
    println!("Aminoácido lido da sequência: {:?}", aminoacido_lido);

    // 4. Usamos 'match' para executar código com base na variante do enum.
    analisar_aminoacido(aminoacido_lido);
    analisar_aminoacido(Aminoacido::Glicina);
}


/// Uma função que usa 'match' para tomar uma decisão com base em um enum.
fn analisar_aminoacido(aa: Aminoacido) {
    print!("Analisando {:?}: ", aa);
    match aa {
        Aminoacido::Alanina => println!("É um aminoácido apolar."),
        Aminoacido::Glicina => println!("É o aminoácido mais simples."),
        Aminoacido::Leucina => println!("É um aminoácido essencial."),
        Aminoacido::Serina  => println!("Possui um grupo hidroxila."),
    }
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Análise de Proteínas e Aminoácidos ---
Proteína encontrada: Proteina { nome: "Albumina", sentido: Forward, comprimento: 585 }
Aminoácido lido da sequência: Leucina
Analisando Leucina: É um aminoácido essencial.
Analisando Glicina: É o aminoácido mais simples.
```

---

### Próximos Passos

No **Dia 12**, vamos mergulhar de cabeça no sistema de **Tratamento de Erros** do Rust, explorando o `Option` em mais detalhes e conhecendo seu poderoso primo, o `Result`.



