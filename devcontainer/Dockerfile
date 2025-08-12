# Passo 1: Use a imagem oficial do Rust baseada no Debian Bookworm.
# ELA JÁ VEM COM RUST (rustup, cargo) INSTALADO em /usr/local/cargo/bin.
FROM rust:bookworm

# Passo 2: Instale as dependências do sistema e ferramentas essenciais como 'root'.
# 'sudo' é necessário para dar permissões ao nosso usuário de desenvolvimento.
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

# Passo 3: Crie um usuário não-root 'vscode' e dê a ele permissões de sudo.
# Esta é a prática padrão e segura para containers de desenvolvimento.
RUN useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# A imagem 'rust' instala o cargo em /usr/local/cargo/bin.
# Adicionamos esta linha explicitamente ao .bashrc do nosso novo usuário.
# Isso FORÇA o shell do Codespaces a saber onde encontrar o 'cargo'
# toda vez que um terminal é aberto.
RUN echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /home/vscode/.bashrc
# -----------------------------------------------------------------------------

# Passo 4: Mude para o nosso novo usuário não-root.
USER vscode

# Passo 5: Instale os componentes do Rust que usaremos no curso.
# Este comando agora funcionará, pois o sistema encontrará 'rustup' no PATH.
RUN rustup component add rustfmt clippy

# Passo 6: Define o diretório de trabalho padrão dentro do container.
WORKDIR /workspaces
