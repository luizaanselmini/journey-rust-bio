# Use a imagem base oficial da Microsoft para desenvolvimento Rust.
# Ela já vem com rustup, cargo, git e outras ferramentas essenciais.
FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Argumento para manter o contexto do usuário.
ARG USERNAME=vscode
USER root

# Atualiza a lista de pacotes e instala as dependências de sistema necessárias.
# Comentários explicam o porquê de cada ferramenta.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    # Ferramentas essenciais de compilação C/C++ e para encontrar bibliotecas.
    build-essential \
    pkg-config \
    # Necessário para a crate 'ring', uma dependência comum em projetos web com crypto.
    libssl-dev \
    # Dependências para a biblioteca HTSlib (manipulação de arquivos SAM/BAM/VCF).
    # Essencial para a crate `rust-htslib` e útil para `rust-bio`.
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    libcurl4-openssl-dev \
    libhts-dev \
    # Dependências para o backend de crates de GUI como `egui`.
    # Mesmo que não se renderize uma janela, as dependências de compilação são necessárias.
    libxkbcommon-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Mude de volta para o usuário não-root.
USER $USERNAME

# Instala componentes essenciais do Rust:
# 'rustfmt' para formatação de código automática.
# 'clippy' para linting e sugestões de melhoria de código.
RUN rustup component add rustfmt clippy
