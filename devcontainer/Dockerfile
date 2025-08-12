# Passo 1: Comece com uma imagem base genérica e estável para Dev Containers.
# Ela já vem com um usuário não-root ('vscode') e 'sudo' configurados.
FROM mcr.microsoft.com/devcontainers/base:bookworm

# Passo 2: Mude para o usuário 'root' para instalar pacotes de sistema.
USER root

# Passo 3: Instale todas as nossas dependências de sistema necessárias.
# Adicionamos 'curl' para poder baixar o instalador do Rust.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    # Ferramentas essenciais
    curl \
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

# Passo 4: Mude de volta para o usuário padrão do container.
USER vscode

# Passo 5: Instale o Rust (rustup, cargo, etc.) para o usuário 'vscode'.
# O script do rustup cuida de configurar o PATH corretamente.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Passo 6: Instale os componentes do Rust necessários para o curso.
RUN rustup component add rustfmt clippy

# Passo 7: Define o diretório de trabalho padrão.
WORKDIR /workspaces
