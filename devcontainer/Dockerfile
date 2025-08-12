# Use a imagem base oficial da Microsoft para desenvolvimento Rust.
FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Argumento para manter o contexto do usuário.
ARG USERNAME=vscode
USER root

# Atualiza a lista de pacotes e instala as dependências de sistema necessárias.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    libcurl4-openssl-dev \
    libhts-dev \
    libxkbcommon-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Mude de volta para o usuário não-root.
USER $USERNAME

# Força a instalação do rustup para o usuário atual para garantir que o PATH e
# o ambiente estejam corretamente configurados. O '-y' automatiza a instalação.
# Isso garante que ~/.cargo/bin seja criado e populado.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Instala os componentes essenciais do Rust: 'rustfmt' e 'clippy'.
# É importante que isso venha DEPOIS da instalação explícita do rustup.
RUN rustup component add rustfmt clippy
