FROM rust:1.74-slim-bullseye

WORKDIR /app

RUN useradd --create-home crustacean \
  && chown -R crustacean:crustacean /app \
  && rustup component add rustfmt \
  && apt-get -y update \
  && apt-get install -y git \
  && rm -rf /var/lib/apt/lists/*

USER crustacean

COPY --chown=crustacean:crustacean . .
