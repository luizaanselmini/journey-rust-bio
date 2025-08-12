# Jornada Rust 🦀 em Bioinformática

![Status da Saga](https://img.shields.io/badge/Saga-Em%20Andamento-brightgreen)
![Linguagem](https://img.shields.io/badge/Linguagem-Rust-orange)
![Licença](https://img.shields.io/badge/Licença-MIT-blue)

Uma jornada pública de 30 dias aprendendo a linguagem de programação Rust do zero, com um foco prático na resolução de problemas de Biologia Computacional e Bioinformática.

---

## 🧬 Sobre o Projeto

Este repositório é o meu diário de bordo digital. Durante 30 dias, dedicarei um tempo para aprender, praticar e documentar minha evolução com a linguagem Rust, aplicando os conceitos diretamente em desafios da bioinformática. A intenção é criar um roteiro estruturado que possa, quem sabe, inspirar e ajudar outros que queiram trilhar um caminho semelhante.

### O que é Bioinformática?

De forma simples, a bioinformática é o campo da ciência que utiliza a computação para entender a biologia. Nós usamos algoritmos e poder computacional para ler, analisar e interpretar o volume massivo de dados gerado por experimentos biológicos. Pense em decifrar um genoma completo, comparar proteínas ou entender como os genes são regulados – são tarefas que seriam impossíveis sem ferramentas computacionais robustas.

### Por que usar Rust para Bioinformática?

A bioinformática exige ferramentas que atendam a um trio de necessidades críticas, e Rust se encaixa perfeitamente nelas:

1.  **🚀 Performance Bruta:** As análises genômicas processam gigabytes (ou terabytes) de dados. Rust compila para código de máquina nativo, oferecendo uma velocidade comparável a C e C++, o que significa que análises que levariam dias podem ser concluídas em horas.

2.  **🛡️ Segurança e Confiabilidade:** Um bug de gerenciamento de memória no meio de uma análise de 48 horas pode ser catastrófico. O sistema de *ownership* e *borrowing* de Rust garante a segurança da memória em tempo de compilação, eliminando uma classe inteira de bugs e tornando os programas incrivelmente robustos e confiáveis. Chega de "segmentation faults"!

3.  **🖥️ Concorrência sem Medo:** Servidores de pesquisa geralmente possuem dezenas de núcleos de CPU. Rust foi projetado para facilitar a escrita de código que aproveita todos esses núcleos de forma segura, permitindo um paralelismo massivo para acelerar ainda mais as análises.

## 🎯 A Jornada: Cronograma de 30 Dias

Cada dia da saga terá seu próprio arquivo de anotações e código, documentando o que foi aprendido.

<details>
<summary><strong>Semana 1: A Base de Rust - Primeiros Passos no Código</strong></summary>

* [**Dia 1:** Olá, Mundo! (Configuração do Ambiente)](/week_1/DIA_01.md)
* [**Dia 2:** Variáveis e Tipos Primitivos](./DIA_02.md)
* [**Dia 3:** Funções, as Enzimas do Código](./DIA_03.md)
* [**Dia 4:** Controle de Fluxo com `if/else`](./DIA_04.md)
* [**Dia 5:** Strings e Manipulação Básica](./DIA_05.md)
* [**Dia 6:** Loops para Repetir Análises](./DIA_06.md)
* [**Dia 7:** Revisão da Semana 1](./DIA_07.md)

</details>

<details>
<summary><strong>Semana 2: Estruturas e Conceitos Essenciais</strong></summary>

* [**Dia 8:** Ownership, o Conceito Central de Rust](./DIA_08.md)
* [**Dia 9:** Borrowing e References](./DIA_09.md)
* [**Dia 10:** Structs para Modelar Dados Biológicos](./DIA_10.md)
* [**Dia 11:** Enums para Estados e Variações](./DIA_11.md)
* [**Dia 12:** `Option` e `Result` para Tratamento de Erros](./DIA_12.md)
* [**Dia 13:** Vetores e Coleções](./DIA_13.md)
* [**Dia 14:** Revisão da Semana 2](./DIA_14.md)

</details>

<details>
<summary><strong>Semana 3: Mergulhando na Bioinformática com Crates</strong></summary>

* [**Dia 15:** Introdução ao `crates.io` e `rust-bio`](./DIA_15.md)
* [**Dia 16:** Lendo Arquivos FASTA](./DIA_16.md)
* [**Dia 17:** Análise de Sequências com `rust-bio`](./DIA_17.md)
* [**Dia 18:** Alinhamento de Sequências (Teoria)](./DIA_18.md)
* [**Dia 19:** Alinhamento na Prática](./DIA_19.md)
* [**Dia 20:** Buscando Motivos (Pattern Matching)](./DIA_20.md)
* [**Dia 21:** Revisão da Semana 3](./DIA_21.md)

</details>

<details>
<summary><strong>Semana 4: Análise de Dados e Interfaces</strong></summary>

* [**Dia 22:** Análise de Dados com Polars - Introdução](./DIA_22.md)
* [**Dia 23:** Lendo CSV com Polars](./DIA_23.md)
* [**Dia 24:** Filtrando e Selecionando Dados](./DIA_24.md)
* [**Dia 25:** Introdução a Interfaces Web com Actix Web](./DIA_25.md)
* [**Dia 26:** Recebendo Dados via Web](./DIA_26.md)
* [**Dia 27:** Introdução a Interfaces Gráficas (GUI) com `egui`](./DIA_27.md)
* [**Dia 28:** Mini-Projeto: Juntando Tudo (Parte 1)](./DIA_28.md)
* [**Dia 29:** Finalizando o Mini-Projeto (Parte 2)](./DIA_29.md)
* [**Dia 30:** Conclusão e Próximos Passos](./DIA_30.md)

</details>

## 📂 Estrutura do Repositório

* Cada dia de aprendizado terá seu próprio arquivo `DIA_XX.md` com anotações, teoria e desafios.
* Os códigos desenvolvidos em cada dia estarão dentro de pastas nomeadas `dia_xx_nome_do_projeto/`.
* Este `README.md` servirá como o índice central da saga.

## 🤝 Como Acompanhar ou Contribuir

Este é um projeto de aprendizado pessoal, mas a jornada é pública! Fique à vontade para:

* **Explorar** os arquivos diários para ver o que aprendi.
* **Sugerir** melhorias, correções ou recursos interessantes abrindo uma **Issue**.
* **Clonar** o repositório e fazer sua própria Jornada!

## 📜 Licença

Este projeto é distribuído sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.
