# Passo 1: Começar com uma imagem Linux minimalista e moderna.
# 'bookworm-slim' é uma versão enxuta do Debian 12.
FROM debian:bookworm-slim

# Passo 2: Evitar que instaladores façam perguntas interativas durante o build.
ENV DEBIAN_FRONTEND=noninteractive

# Passo 3: Instalar, como 'root', as dependências de sistema essenciais.
# Isso inclui ferramentas para compilar, gerenciar código e bibliotecas C
# que são necessárias para as crates de bioinformática.
RUN apt-get update && apt-get -y install --no-install-recommends \
    # Ferramentas básicas de desenvolvimento e para baixar o Rust
    build-essential \
    curl \
    git \
    sudo \
    # Bibliotecas C para bioinformática (arquivos BAM/VCF, compressão, etc)
    pkg-config \
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    libcurl4-openssl-dev \
    libhts-dev \
    # Bibliotecas C para o backend de algumas crates de GUI
    libxkbcommon-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    # Limpa o cache para manter a imagem final pequena.
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Passo 4: Criar um usuário não-root 'vscode' para o desenvolvimento.
# É uma prática de segurança padrão não trabalhar como 'root'.
# Damos a ele permissão de 'sudo' sem precisar de senha.
RUN useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Passo 5: Mudar para o novo usuário não-root.
USER vscode
WORKDIR /home/vscode

# Passo 6: Instalar a toolchain completa do Rust (rustc, cargo, rustup).
# O script é baixado e executado. A flag '-y' automatiza a instalação,
# aceitando as opções padrão sem precisar de interação.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Passo 7: Adicionar os componentes 'rustfmt' (para formatação) e 'clippy' (para linting).
# Executamos usando o caminho completo para garantir que funcione, pois o PATH
# no shell de build pode não ter sido atualizado ainda.
RUN /home/vscode/.cargo/bin/rustup component add rustfmt clippy

# Passo 8: Forçar a adição do PATH ao .bashrc.
# Esta é a garantia final de que qualquer novo terminal interativo
# saberá onde encontrar o 'cargo'.
RUN echo 'export PATH="/home/vscode/.cargo/bin:$PATH"' >> /home/vscode/.bashrc

# Passo 9: Definir o diretório de trabalho final para o workspace.
WORKDIR /workspaces
