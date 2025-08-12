# Passo 1: Use a imagem oficial do Rust. Ela já vem com a toolchain do Rust instalada.
FROM rust:bookworm

# Passo 2: Como 'root', instale as dependências de sistema e a ferramenta 'sudo'.
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    # 'sudo' é necessário para dar permissões ao nosso usuário de desenvolvimento.
    sudo \
    # Ferramentas de compilação C/C++ que muitas crates do Rust utilizam.
    build-essential \
    pkg-config \
    # Dependências comuns para bioinformática e desenvolvimento web.
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    libcurl4-openssl-dev \
    libhts-dev \
    # Dependências para o backend de algumas bibliotecas de GUI.
    libxkbcommon-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    # Limpa o cache do apt para manter a imagem final menor.
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Passo 3: Crie um usuário não-root 'vscode' e adicione-o ao grupo 'sudo'.
RUN useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Passo 4: Adicione explicitamente o caminho do Rust ao .bashrc do novo usuário.
# Esta é a correção crucial para garantir que o terminal interativo do Codespaces encontre o 'cargo'.
RUN echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /home/vscode/.bashrc

# Passo 5: Mude para o novo usuário não-root.
USER vscode

# Passo 6: Instale os componentes adicionais do Rust que usaremos no curso.
# Este comando funciona porque 'rustup' já está na imagem e o PATH está correto.
RUN rustup component add rustfmt clippy

# Passo 7: Define o diretório de trabalho padrão.
WORKDIR /workspaces
