# Jornada Rust 🦀 em Bioinformática - DIA 1

## Título: Começando a Aventura! Instalação e o Primeiro "Olá, Bioinformática!"

Bem-vindo(a) ao primeiro dia da minha jornada de 30 dias aprendendo Rust com foco em biologia computacional! Hoje, o objetivo é preparar o terreno: entender por que Rust é uma escolha tão promissora para a bioinformática, instalar todas as ferramentas necessárias e, claro, fazer nosso primeiro programa funcionar. A jornada de mil linhas de código começa com um único `cargo run`!

---

### 🎯 Objetivos do Dia

1.  **Entender** por que Rust é relevante para a bioinformática.
2.  **Instalar** o compilador Rust e suas ferramentas (`rustup`, `cargo`).
3.  **Configurar** o ambiente de desenvolvimento (VS Code).
4.  **Criar e executar** o primeiro programa em Rust.

---

### 🧠 Conceitos do Dia (Teoria - 5 min)

#### Por que Rust para Bioinformática?

A bioinformática lida com volumes massivos de dados. Pense em sequenciar um genoma humano: são bilhões de bases de DNA que precisam ser lidas, alinhadas, analisadas e comparadas. Para isso, precisamos de programas que sejam:

1.  **Rápidos:** A velocidade é crucial. Queremos que nossas análises terminem em horas, não em semanas. Rust compila para código de máquina nativo, atingindo velocidades comparáveis a C e C++.
2.  **Seguros:** Um bug que corrompe a memória no meio de uma análise de 24 horas é um desastre. O famoso sistema de "Ownership" (posse) de Rust garante a segurança da memória em tempo de compilação, eliminando uma classe inteira de bugs comuns em outras linguagens.
3.  **Concorrentes:** Computadores modernos têm múltiplos núcleos de processamento. Rust foi projetado desde o início para facilitar a escrita de código que executa tarefas em paralelo, acelerando drasticamente as análises.

#### O Kit de Ferramentas: `rustup` e `cargo`

Ao instalar Rust, você na verdade instala o `rustup`, que é um instalador e gerenciador de versões do Rust. Ele, por sua vez, instala duas ferramentas essenciais:

* **`rustc`:** O compilador de Rust. Ele transforma seu código-fonte em um programa executável.
* **`cargo`:** O canivete suíço do ecossistema Rust. Ele é o **gerenciador de projetos**, **gerenciador de dependências** (baixa e compila bibliotecas externas, chamadas de "crates") e **orquestrador de compilação**. Na prática, você usará o `cargo` para quase tudo: criar, construir, testar e executar seus projetos.

---

### 💻 Mão na Massa (Prática - 10 min)

Vamos criar nosso primeiro programa! Siga estes passos no seu terminal.

#### Passo 1: Instalar Rust

Se ainda não o fez, visite [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) e execute o comando de instalação para seu sistema operacional. Geralmente, é algo assim:

```bash
# Para Linux e macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Aparecerá uma lista de opções, aperte `Enter`:
```
1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>
```
Siga as instruções na tela. Após a conclusão, abra um **novo terminal** e verifique se a instalação funcionou:

```bash
cargo --version
# Você deve ver algo como: cargo 1.79.0 (2d8ce2948 2024-05-08)
```

#### Passo 2: Configurar o VS Code (Recomendado)

1.  Instale o [Visual Studio Code](https://code.visualstudio.com/).
2.  Na aba de Extensões (Extensions), procure e instale a extensão **`rust-analyzer`**. Ela fornecerá autocompletar, detecção de erros em tempo real e muitas outras funcionalidades úteis.

#### Passo 2.1: Usar o Github Codespace
Você pode optar por não fazer uma instalção nativa do RUST, para isso use o GitHub Codespace com uma instalação minimalista do Linux Debian para seguir na Jornada.

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new/mlfalco-bioinfo/journey-rust-bio)

Siga o Passo 1, para instalação do RUST no seu Codespace, antes do comando de verificação de instalação `cargo --version`, realize primeiro esse passo para configurar o RUST no seu `$HOME`.

Assim que finalizar a instalação do RUST, cole o seguinte comando no terminal:
```
. "$HOME/.cargo/env"
```
**Pronto!**
O RUST está instalado e pronto para ser usado, teste o comando abaixo para verificar a instalação.

**Agora, é só aproveitar a Jornada!**

#### Passo 3: Criar seu Primeiro Projeto

O `cargo` facilita a criação de uma estrutura de projeto padrão.

```bash
# Navegue até a pasta onde você guardará seus projetos
# Ex: cd Documentos/ProjetosRust

# Peça ao cargo para criar um novo projeto chamado "hello_bio"
cargo new hello_bio
```

O `cargo` criará uma nova pasta `hello_bio` com a seguinte estrutura:

```
hello_bio/
├── .git          # Inicializa um repositório git
├── .gitignore    # Um .gitignore padrão para projetos Rust
├── Cargo.toml    # O arquivo de manifesto do projeto
└── src/          # A pasta com o código-fonte
    └── main.rs   # O arquivo principal do seu programa
```

* **`Cargo.toml`**: Contém metadados sobre seu projeto, como nome, versão e, mais importante, suas dependências (as "crates").
* **`src/main.rs`**: Onde a mágica acontece! Este é o ponto de entrada do seu programa.

#### Passo 4: Escrever e Executar o Código

1.  Abra a pasta `hello_bio` no VS Code.
2.  Abra o arquivo `src/main.rs`. Ele já virá com um "Hello, world!".
3.  Vamos modificá-lo para nossa Jornada (use o Nano, Vim ou outro editor de texto). Substitua o conteúdo pelo seguinte:

```rust
// src/main.rs

// A função main é o ponto de entrada de todo programa executável em Rust.
fn main() {
    // A macro println! imprime texto na console.
    // O '!' indica que estamos chamando uma macro, não uma função comum.
    println!("Olá, Bioinformática! Dia 1 da minha jornada Rust.");
}
```

4.  Agora, de volta ao terminal (você pode usar o terminal integrado do VS Code), dentro da pasta `hello_bio`, execute o programa:

```bash
# Este comando compila e executa o projeto
cargo run
```

#### Saída Esperada

Se tudo deu certo, você verá a seguinte saída no seu terminal:

```
   Compiling hello_bio v0.1.0 (/path/to/your/project/hello_bio)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_bio`
Olá, Bioinformática! Dia 1 da minha jornada Rust.
```

**Parabéns! Você acabou de escrever e executar seu primeiro programa em Rust!**

---

### 💡 Para o Blog e GitHub

**Snippet de Código do Dia:**

```rust
fn main() {
    println!("Olá, Bioinformática! Dia 1 da minha jornada Rust.");
}
```

**Sugestões para post no LinkedIn/Blog:**

> **Título:** Começando a Aventura! Dia 1 da minha Saga de 30 Dias com Rust para Bioinformática.
>
> **Corpo:** "Hoje dei o primeiro passo em uma jornada que estou super animado(a) para compartilhar: aprender Rust com foco em Bioinformática! 🚀
>
> Por que Rust? Pela promessa de performance de C++, mas com garantias de segurança que evitam dores de cabeça com bugs de memória. Para uma área que lida com terabytes de dados genômicos, isso é revolucionário!
>
> O objetivo de hoje foi simples, mas fundamental: configurar o ambiente e rodar o primeiro "Olá, Mundo!". Ver a mensagem `Olá, Bioinformática! 🦀` no terminal foi o ponto de partida perfeito.
>
> A jornada será documentada diariamente. Se você tem interesse em programação de alta performance, bioinformática ou apenas quer acompanhar um desafio de aprendizado, siga por aqui!
>
> #RustLang #Bioinformatics #ComputationalBiology #30DayChallenge #LearnInPublic"

---

### 🤔 Reflexão e Próximos Passos

O primeiro dia foi sobre quebrar a inércia. Instalamos as ferramentas e vimos que o `cargo` torna o processo de iniciar um projeto incrivelmente simples. Amanhã, no **Dia 2**, vamos mergulhar nos blocos de construção fundamentais de qualquer programa: **variáveis e tipos de dados**, e como eles se aplicam ao nosso universo biológico.
