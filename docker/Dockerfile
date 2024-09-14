FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    python3 \
    python3-pip \
    rustc \
    cargo \
    g++ \
    unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://bun.sh/install | bash

ENV PATH="/root/.bun/bin:$PATH"

RUN python3 -m pip install --upgrade pip

WORKDIR /app

CMD ["/bin/bash"]