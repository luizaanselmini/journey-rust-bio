# Passo 1: Começar com uma imagem Linux minimalista e moderna.
FROM debian:bookworm-slim

# Passo 2: Evitar que instaladores façam perguntas interativas durante o build.
ENV DEBIAN_FRONTEND=noninteractive

# Passo 3: Instalar, como 'root', as dependências de sistema essenciais.
# O Rust NÃO será instalado aqui. Apenas as ferramentas de suporte.
RUN apt-get update && apt-get -y install --no-install-recommends \
    # Ferramentas básicas de desenvolvimento e 'curl' (que você usará no Dia 1)
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
RUN useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Passo 5: Mude para o novo usuário não-root.
USER vscode

# Passo 6: Definir o diretório de trabalho final para o workspace.
WORKDIR /workspaces

# FIM DO DOCKERFILE. A instalação do Rust será feita manualmente pelo usuário.
