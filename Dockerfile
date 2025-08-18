# =======================
# Builder
# =======================
FROM rustlang/rust:nightly-bookworm as builder

# Instalar cargo-binstall
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin

# Instalar herramientas necesarias
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends clang curl build-essential \
    && apt-get clean -y

# Instalar Node.js (LTS)
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs

# Instalar cargo-leptos
RUN cargo binstall cargo-leptos -y

# Agregar target WASM
RUN rustup target add wasm32-unknown-unknown

# Copiar proyecto
WORKDIR /app
COPY . .

# Instalar dependencias JS (Tailwind)
RUN npm install

# Construir la app (SSR + CSS/WASM)
RUN cargo leptos build --release -vv

# =======================
# Runtime
# =======================
FROM debian:bookworm-slim
WORKDIR /app

# Instalar dependencias mínimas
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates curl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copiar binario y site
COPY --from=builder /app/target/release/portfolio /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

# Copiar archivo .env
COPY .env /app/.env

# Variables de entorno
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 8080

# Ejecutar binario
CMD ["/app/portfolio"]