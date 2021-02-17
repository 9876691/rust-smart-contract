FROM ianpurton/rust-devcontainer:latest AS base

FROM base AS development

COPY ./bash_history /home/vscode/.bash_history
RUN sudo chown vscode /home/vscode/.bash_history

# Install Latex 
RUN sudo apt-get update && sudo apt-get -y install texlive-latex-extra latexmk 

RUN cargo install --git https://github.com/hicommonwealth/cargo-contract cargo-contract --force \
    && rustup toolchain install nightly \
    && rustup target add wasm32-unknown-unknown --toolchain nightly

RUN rustup toolchain install nightly --component rust-src

