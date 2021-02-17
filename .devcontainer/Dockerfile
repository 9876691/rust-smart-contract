FROM ianpurton/rust-devcontainer:latest AS base

FROM base AS development

COPY ./bash_history /home/vscode/.bash_history
RUN sudo chown vscode /home/vscode/.bash_history

# Install stuff needed to build https://github.com/paritytech/canvas-node
RUN sudo apt-get update && sudo apt-get -y install texlive-latex-extra latexmk clang libclang-dev llvm

# Add nightly to the toolchain.
RUN cargo install --git https://github.com/hicommonwealth/cargo-contract cargo-contract --force \
    && rustup toolchain install nightly \
    && rustup target add wasm32-unknown-unknown --toolchain nightly
    && rustup toolchain install nightly --component rust-src

# The noide we can run to deploy contracts.
RUN cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.4 --force --locked
