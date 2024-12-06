# Этап сборки: используем официальный образ Rust
FROM rust:1.83 as builder

# Рабочая директория в контейнере
WORKDIR /usr/src/app

RUN apt-get update && apt-get install --no-install-recommends -y \
    llvm \
    clang \
    libclang-dev \
    pkg-config \
    libssl-dev \
    libopencv-dev \
    libclang-dev \
    && ldconfig -v \
    && rm -rf /var/lib/apt/lists/*

# Копируем файлы Cargo.toml и Cargo.lock для того, чтобы сначала собрать зависимости
COPY Cargo.toml ./

# Копируем исходный код проекта в контейнер
COPY src ./src

# Собираем проект и его зависимости
RUN cargo build --release

# Этап финальной сборки: используем более легкий образ Debian
FROM debian:bookworm-slim

# Устанавливаем зависимости, необходимые для выполнения программы
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Копируем собранный бинарный файл из первого этапа сборки
COPY --from=builder /usr/src/app/target/release/camera-service-rs /usr/local/bin/camera-service-rs

# Копируем конфигурационные файлы, если они есть
COPY config.yml /usr/local/bin/config.yml
COPY camera_config.yml /usr/local/bin/camera_config.yml

# Устанавливаем переменную окружения для уровня логирования
ENV RUST_LOG=debug

# Открываем порт для приложения
EXPOSE 8002

# Запускаем приложение
CMD ["camera-service-rs"]
