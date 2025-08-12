# Passo 1: Use a imagem oficial do Rust. A premissa é que ela JÁ TEM RUST instalado.
FROM rust:bookworm

# --- INÍCIO DA SEÇÃO DE DEPURAÇÃO ---

# DEBUG [como root]: Vamos inspecionar a instalação de Rust que vem na imagem.
RUN echo "--- [DEBUG como ROOT] Verificando a instalação padrão da imagem 'rust:bookworm' ---"
RUN echo "--- [DEBUG como ROOT] Conteúdo de /usr/local/cargo/bin: ---" && ls -la /usr/local/cargo/bin
RUN echo "--- [DEBUG como ROOT] Versão do rustc usando caminho absoluto: ---" && /usr/local/cargo/bin/rustc --version
RUN echo "--- [DEBUG como ROOT] Variável PATH padrão: ---" && echo $PATH

# Passo 2: Mude para 'root' (já estamos, mas é uma boa prática ser explícito) para instalar pacotes.
USER root

# Passo 3: Instale as dependências do sistema.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    sudo \
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

# Passo 4: Crie o usuário 'vscode' para o Codespaces.
RUN useradd -m -s /bin/bash vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# DEBUG [como root]: Adiciona o PATH ao .bashrc do novo usuário para garantir que shells interativos funcionem.
RUN echo "--- [DEBUG como ROOT] Forçando o PATH no .bashrc do usuário 'vscode'... ---"
RUN echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /home/vscode/.bashrc
RUN echo "--- [DEBUG como ROOT] Conteúdo final do .bashrc: ---" && tail -n 3 /home/vscode/.bashrc

# Passo 5: Mude para o novo usuário 'vscode'.
USER vscode
WORKDIR /home/vscode

# DEBUG [como vscode]: Agora como 'vscode', vamos ver o que o ambiente herdou durante o build.
RUN echo "--- [DEBUG como VSCODE] Verificando o ambiente do build... ---"
RUN echo "--- [DEBUG como VSCODE] Eu sou o usuário: $(whoami) ---"
RUN echo "--- [DEBUG como VSCODE] Meu PATH durante o build é: $(echo $PATH) ---"

# DEBUG [como vscode]: O teste de fogo. Se este comando falhar, o PATH não foi herdado corretamente no build.
RUN echo "--- [DEBUG como VSCODE] Tentando executar 'rustc --version' diretamente... ---"
RUN rustc --version

# --- FIM DA SEÇÃO DE DEPURAÇÃO ---

# Passo 6: Instale os componentes do Rust que usaremos no curso.
RUN rustup component add rustfmt clippy

# Passo 7: Define o diretório de trabalho final.
WORKDIR /workspaces
