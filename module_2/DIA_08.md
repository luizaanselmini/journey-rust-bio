# Jornada Rust 🦀 em Bioinformática - DIA 8

## O Dono da Memória: Introdução à Posse (Ownership)

Bem-vindo à segunda semana da nossa jornada! Na primeira semana, construímos uma base sólida com os fundamentos da programação. Agora, vamos explorar o superpoder do Rust, o conceito que garante sua segurança e performance sem a necessidade de um *garbage collector*: o sistema de **Posse (Ownership)**.

Até agora, quando passamos dados, não nos preocupamos muito com "quem era o dono" daquela informação. O Rust, no entanto, se preocupa. E muito. Cada pedaço de dado na memória tem um "dono" claro e único, assim como um equipamento caro no laboratório que pertence a um pesquisador específico.

Este sistema de posse é como o Rust gerencia a memória do computador. Entendê-lo é o passo mais crucial para se tornar um programador Rust proficiente.

---

### Objetivos do Dia

1.  **Entender** o que é o sistema de Posse (Ownership) e por que ele é importante.
2.  **Aprender** as três regras fundamentais da Posse.
3.  **Diferenciar** entre a semântica de **Mover (Move)** e a de **Copiar (Copy)**.
4.  **Ver** na prática como a posse de uma `String` é transferida.

---

### Conceitos do Dia

O sistema de Posse é um conjunto de regras que o compilador do Rust verifica em tempo de compilação. Nenhuma dessas regras afeta a velocidade do seu programa em tempo de execução.

#### As Três Regras da Posse

Todo o sistema pode ser resumido em três regras:

1.  Cada valor em Rust tem um "dono" (*owner*).
2.  Só pode haver um dono por vez.
3.  Quando o dono sai de escopo, o valor é "descartado" (*dropped*) e a memória é liberada.

#### A Transferência de Posse: Mover (Move)

Vamos ver isso com um tipo de dado que já conhecemos e que vive no *heap*: a `String`.

```rust
let s1 = String::from("sequencia");
let s2 = s1; // A posse é "movida" de s1 para s2
```

No momento em que fazemos `let s2 = s1;`, Rust não faz uma cópia completa dos dados da string (o que seria caro). Em vez disso, ele faz uma **transferência de posse**.

* **Analogia:** Pense na `s1` como a única chave física que abre um sequenciador de DNA no laboratório. Quando você faz `let s2 = s1;`, você não duplica a máquina; você entrega a **única chave existente** para `s2`. Agora, `s1` não tem mais a chave e não pode mais acessar a máquina.

Para garantir a segurança da memória (evitando que duas variáveis tentem liberar a mesma memória), o Rust **invalida `s1`** após a transferência.

```rust
let s1 = String::from("sequencia");
let s2 = s1;

// A linha abaixo causaria um ERRO DE COMPILAÇÃO!
// s1 não é mais um dono válido.
println!("s1 = {}", s1); 
```
Este mecanismo é chamado de **move (mover)**.

#### A Exceção à Regra: Copiar (Copy)

"Então eu não posso nem copiar um inteiro?", você pode se perguntar. Pode!

Para tipos de dados simples e de tamanho fixo que vivem inteiramente na *stack* (como `i32`, `f64`, `bool`, `char`), Rust não move, ele **copia** o valor. A cópia desses dados é tão rápida e barata que não há razão para invalidar a variável original.

Esses tipos possuem uma característica (um `trait`) especial chamada `Copy`.

```rust
let x = 5;
let y = x; // O valor de x é copiado para y

// Ambas as variáveis são válidas, pois i32 implementa o trait 'Copy'.
println!("x = {}, y = {}", x, y); 
```
**Regra geral:** Se o tipo de dado vive na *stack*, ele geralmente é `Copy`. Se ele gerencia memória no *heap* (como `String` e `Vec`), ele não é `Copy` e será movido.

---

### HANDS-ON

Vamos ver o compilador em ação! Vamos testar a diferença entre `Copy` e `Move`.

**1. Crie o projeto do Dia 8:**

```bash
cargo new dia8_ownership
cd dia8_ownership
```

**2. Edite o arquivo `src/main.rs`:**

Cole o código abaixo. Preste atenção especial na linha comentada que causaria um erro.

```rust
// src/main.rs

fn main() {
    // --- Exemplo com o trait 'Copy' (tipos da stack) ---
    println!("--- Testando o Comportamento de Cópia (Copy) ---");
    let x = 100; // x é do tipo i32, que implementa 'Copy'
    let y = x;   // O valor de x é copiado para y.

    // Ambas as variáveis continuam válidas e podem ser usadas.
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
    println!("Ambas as variáveis são válidas após a atribuição.");

    println!("\n-------------------------------------------------\n");

    // --- Exemplo com a semântica de 'Move' (tipos do heap) ---
    println!("--- Testando o Comportamento de Mover (Move) ---");
    let s1 = String::from("GATTACA"); // s1 é dona da String "GATTACA"
    let s2 = s1;                      // A posse da String é MOVIDA de s1 para s2.

    // s1 não é mais a dona e foi invalidada para garantir a segurança da memória.
    
    // A linha abaixo, se descomentada, causará um erro de compilação!
    // TENTE DESCOMENTAR PARA VER A MENSAGEM DO COMPILADOR!
    // println!("O valor de s1 é: {}", s1);

    // Apenas s2, a nova dona, pode ser usada.
    println!("O valor de s2 é: {}", s2);
    println!("Apenas a nova dona (s2) é válida após a atribuição.");
}
```

**3. Execute o código:**

Primeiro, execute com a linha comentada.

```bash
cargo run
```

**Saída Esperada:**
```
--- Testando o Comportamento de Cópia (Copy) ---
O valor de x é: 100
O valor de y é: 100
Ambas as variáveis são válidas após a atribuição.

-------------------------------------------------

--- Testando o Comportamento de Mover (Move) ---
O valor de s2 é: GATTACA
Apenas a nova dona (s2) é válida após a atribuição.
```

**4. O Experimento:**
Agora, volte ao código, descomente a linha `// println!("O valor de s1 é: {}", s1);` e tente executar `cargo run` novamente. Veja a famosa e útil mensagem de erro do compilador do Rust:
 
 **`error[E0382]: borrow of moved value: s1`**  
 
 Ele está te protegendo!

---

### Próximos Passos

No **Dia 9**: **Empréstimos (Borrowing) e Referências**, a forma de permitir que outras partes do seu código *usem* seus dados sem precisar transferir a posse.
