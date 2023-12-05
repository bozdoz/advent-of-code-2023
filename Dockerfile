FROM rust:1.73-slim-bullseye

WORKDIR /app

RUN useradd --create-home crustacean \
  && chown -R crustacean:crustacean /app \
  && rustup component add rustfmt

USER crustacean

COPY --chown=crustacean:crustacean . .
