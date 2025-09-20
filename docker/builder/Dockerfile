FROM rust:1-slim
ARG USERNAME=appuser
ARG UID=1000
ARG GROUPNAME=appgroup
ARG GID=1000

# Install necessary build tools for Dioxus web apps
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt update && apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

RUN groupadd -g ${GID} ${GROUPNAME} 
RUN useradd -m -u ${UID} -g ${GROUPNAME} ${USERNAME}
COPY --chown=${UID}:${GID} ./rust/ /app/
RUN rm -rf /app/target

USER ${USERNAME}
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Set the working directory inside the container
WORKDIR /app

RUN cargo binstall dioxus-cli

SHELL ["/bin/bash", "-c"]
