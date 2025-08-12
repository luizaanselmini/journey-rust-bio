# Passo 1: Use a imagem oficial do Rust baseada no Debian Bookworm.
# Esta imagem é minimalista e vem com rustup, cargo e o PATH já configurados.
FROM rust:bookworm

# Passo 2: Instale as dependências do sistema e ferramentas essenciais como 'root'.
# Adicionamos 'sudo' para dar permissões elevadas ao nosso usuário não-root.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    # Ferramentas essenciais
    sudo \
    build-essential \
    pkg-config \
    # Dependências para crates de bioinformática e web
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    libcurl4-openssl-dev \
    libhts-dev \
    # Dependências para crates de GUI
    libxkbcommon-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    # Limpa o cache para manter a imagem pequena
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Passo 3: Crie um usuário não-root chamado 'vscode' para o ambiente de desenvolvimento.
# Adicionamos o usuário ao grupo 'sudo' e permitimos que ele use sudo sem senha.
# Esta é uma prática padrão e segura para containers de desenvolvimento.
RUN useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Passo 4: Mude para o nosso novo usuário não-root.
# A partir daqui, todos os comandos são executados como 'vscode'.
USER vscode

# Passo 5: Instale os componentes do Rust necessários para o curso.
# 'rustfmt' para formatação e 'clippy' para linting.
RUN rustup component add rustfmt clippy

# Passo 6: Define o diretório de trabalho padrão.
WORKDIR /workspaces
