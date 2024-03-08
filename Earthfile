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
  RUN micromamba run -n app bash -c 'cargo bin --install && cargo bin --sync-aliases'
  SAVE IMAGE --cache-hint
  RUN micromamba run -n app cargo cmd coverage-llvm
  WORKDIR /

deps:
  FROM +micromamba
  WORKDIR /build/app
  COPY conda-lock.yml .
  RUN micromamba create -y -n app -f conda-lock.yml
  ENV PATH=$PATH:/root/.cargo/bin
  # TODO: replace this curl | bash
  RUN micromamba run -n app bash -c 'curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash'
  RUN micromamba run -n app cargo binstall -y cargo-run-bin@1.7.2
  WORKDIR /
  SAVE IMAGE --cache-hint

micromamba:
  FROM +upstream
  ENV MAMBA_ROOT_PREFIX=/build/micromamba
  RUN curl -Ls https://micro.mamba.pm/api/micromamba/linux-64/latest | tar -xvj bin/micromamba -C /usr/local
