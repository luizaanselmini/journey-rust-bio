# Jornada Rust 🦀 em Bioinformática - DIA 7

## Revisão da Semana 1: Construindo a Primeira Ferramenta

Chegamos ao final da nossa primeira semana! Foi uma jornada intensa onde saímos do zero e montamos um laboratório de desenvolvimento funcional. Revisitamos conceitos de programação sob a ótica do Rust e começamos a construir nosso arsenal de ferramentas.

Nesta primeira semana, nós aprendemos a:
* **Dia 1:** Configurar o ambiente.
* **Dia 2:** Armazenar dados com **variáveis** e **tipos**.
* **Dia 3:** Organizar o código com **funções**.
* **Dia 4:** Tomar decisões com **controle de fluxo** (`if/else`).
* **Dia 5:** Manipular texto com **`String`** e **`&str`**.
* **Dia 6:** Automatizar tarefas com **loops**.

Hoje, vamos agir como um pesquisador que combina várias técnicas para obter um resultado final. Vamos juntar todos esses conceitos para construir nossa primeira ferramenta de bioinformática completa.

---

### Objetivo do Dia

* **Consolidar** os conhecimentos da primeira semana aplicando-os em um único programa.
* **Praticar** a criação de um programa modular com múltiplas funções.
* **Construir** uma ferramenta de linha de comando que realiza duas análises em uma sequência de DNA.

---

### Desafio: Ferramenta de Análise de Sequência

Vamos construir um programa que:
1.  Recebe uma sequência de DNA como entrada (uma `String`).
2.  Chama uma função para **transcrever** essa sequência de DNA para RNA.
3.  Chama uma segunda função para **calcular o conteúdo GC** da sequência de DNA *original*.
4.  Imprime um relatório formatado com os resultados.

---

### HANDS-ON

Este projeto integrará tudo o que vimos. A `main` atuará como nossa orquestradora, e as funções especialistas farão o trabalho pesado.

**1. Crie o projeto do Dia 7:**

```bash
cargo new dia7_revisao_semana1
cd dia7_revisao_semana1
```

**2. Edite o arquivo `src/main.rs`:**

Substitua o conteúdo pelo código completo abaixo. Leia os comentários para ver como cada conceito da semana está sendo aplicado.

```rust
// src/main.rs

// A função 'main' orquestra todo o processo.
fn main() {
    println!("--- Ferramenta de Análise de Sequência em Rust ---");

    // Dia 2: Declarando uma variável do tipo String.
    let sequencia_dna = String::from("GATTACCA");

    println!("\nSequência de DNA para análise: {}", sequencia_dna);
    println!("-------------------------------------------");

    // Dia 3: Chamando nossa primeira função especialista.
    let sequencia_rna = transcrever_para_rna(&sequencia_dna);
    println!("Sequência de RNA Transcrita: {}", sequencia_rna);

    // Dia 3: Chamando nossa segunda função especialista.
    let conteudo_gc = calcular_conteudo_gc(&sequencia_dna);
    // Usamos :.2 para formatar o float com duas casas decimais.
    println!("Conteúdo GC da sequência de DNA: {:.2}%", conteudo_gc * 100.0);

    println!("-------------------------------------------");
    println!("Análise concluída.");
}


/// Função especialista 1: Transcreve uma sequência de DNA para RNA.
/// Dia 3: Recebe um parâmetro do tipo &str (empréstimo).
/// Dia 5: Retorna um novo dado do tipo String (transferência de posse).
fn transcrever_para_rna(dna: &str) -> String {
    // Dia 2 e 5: Criação de uma String mutável e vazia.
    let mut rna = String::new();

    // Dia 6: Usando um loop 'for' para iterar sobre cada caractere.
    for base in dna.chars() {
        // Dia 4: Usando 'match' (uma forma de controle de fluxo) para tomar decisões.
        let base_rna = match base {
            'A' => 'U',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => 'N', // Caractere desconhecido
        };
        // Dia 5: Adicionando o caractere resultante à nossa String de RNA.
        rna.push(base_rna);
    }
    rna // Retorno da String recém-criada.
}


/// Função especialista 2: Calcula a proporção de 'G' e 'C' em uma sequência.
/// Dia 3: Recebe um &str e retorna um f64.
fn calcular_conteudo_gc(dna: &str) -> f64 {
    let mut gc_count = 0.0; // Usamos f64 para evitar conversões depois.
    let mut total_count = 0.0;

    // Dia 6: Loop para analisar a sequência.
    for base in dna.chars() {
        // Dia 4: Controle de fluxo com 'if/else if'.
        if base == 'G' || base == 'C' {
            gc_count += 1.0;
            total_count += 1.0;
        } else if base == 'A' || base == 'T' {
            total_count += 1.0;
        }
    }

    // Evita divisão por zero se a sequência estiver vazia ou for inválida.
    if total_count == 0.0 {
        return 0.0;
    }
    
    // Retorna a proporção.
    gc_count / total_count
}
```

**3. Execute o código:**

```bash
cargo run
```

**Saída Esperada:**

```
--- Ferramenta de Análise de Sequência em Rust ---

Sequência de DNA para análise: GATTACCA
-------------------------------------------
Sequência de RNA Transcrita: CUAAUGGU
Conteúdo GC da sequência de DNA: 37.50%
-------------------------------------------
Análise concluída.
```

---

### Próximos Passos

Começaremos o **Dia 8** explorando o pilar central da linguagem: o sistema de **Posse (Ownership)**.
