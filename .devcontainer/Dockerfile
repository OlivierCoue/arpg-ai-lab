FROM mcr.microsoft.com/devcontainers/base:trixie

# Install uv
RUN curl -LsSf -o /tmp/uv-installer.sh https://astral.sh/uv/install.sh \
    && env UV_INSTALL_DIR=/usr/local/bin sh /tmp/uv-installer.sh \
    && rm /tmp/uv-installer.sh \
    && uv --version

# Install Rust stable with the repository's required components
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable \
    && rustup component add rustfmt clippy \
    && chmod -R a+rwX ${RUSTUP_HOME} ${CARGO_HOME} \
    && rustc --version \
    && cargo --version

# Install GitHub Copilot CLI
RUN curl -fsSL https://gh.io/copilot-install | bash

ENV COPILOT_MODEL=mai-code-1.1-flash

# Bevy required dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    lld clang cmake llvm \
    gcc-mingw-w64 g++-mingw-w64 \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev

RUN rustup target add x86_64-pc-windows-gnu