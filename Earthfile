VERSION 0.8
FROM scratch
ARG --global REGISTRY="ghcr.io"
ARG --global PROJECT="nalabelle/build"

upstream:
  FROM ghcr.io/nalabelle/build/images/debian:0.1.0-debian12@sha256:6b646df955afd19ca4dc75257c9978da9d884f4286dd932972dadc183d0f8fca
  RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive \
        apt-get install -y --no-install-recommends \
          bzip2 \
    && rm -rf /var/lib/apt/lists/*

test:
  FROM +deps
  WORKDIR /build/app
  COPY Cargo.lock Cargo.toml .
  COPY --dir src .
  RUN micromamba run -n app cargo test
  WORKDIR /

deps:
  FROM +micromamba
  WORKDIR /build/app
  COPY conda-lock.yml .
  RUN micromamba create -y -n app -f conda-lock.yml
  WORKDIR /
  SAVE IMAGE --cache-hint

micromamba:
  FROM +upstream
  ENV MAMBA_ROOT_PREFIX=/build/micromamba
  RUN curl -Ls https://micro.mamba.pm/api/micromamba/linux-64/latest | tar -xvj bin/micromamba -C /usr/local
