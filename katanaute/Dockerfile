# Katanaute Dockerfile - Phoenix + React Build
#
# This Dockerfile builds both the Phoenix backend and React frontend.
# It MUST be run from the monorepo root (not from inside katanaute/) so the
# build context includes both katanaute/ and katareact/ directories.
#
# Build from monorepo root:
#   docker build -f katanaute/Dockerfile -t katanaute .
#
# The build process:
#   1. Installs Elixir, Node.js, and dependencies
#   2. Compiles Phoenix application
#   3. Runs `mix react.build` to build and deploy React frontend
#   4. Compiles Phoenix assets
#   5. Creates production release
#
# Find eligible builder and runner images on Docker Hub. We use Ubuntu/Debian
# instead of Alpine to avoid DNS resolution issues in production.
#
# https://hub.docker.com/r/hexpm/elixir/tags?name=ubuntu
# https://hub.docker.com/_/ubuntu/tags
#
# This file is based on these images:
#
#   - https://hub.docker.com/r/hexpm/elixir/tags - for the build image
#   - https://hub.docker.com/_/debian/tags?name=bookworm-20251103-slim - for the release image
#   - https://pkgs.org/ - resource for finding needed packages
#   - Ex: docker.io/hexpm/elixir:1.19.3-erlang-28.1.1-debian-bookworm-20251103-slim
#
ARG ELIXIR_VERSION=1.19.3
ARG OTP_VERSION=28.1.1
ARG DEBIAN_VERSION=bookworm-20251103-slim

ARG BUILDER_IMAGE="docker.io/hexpm/elixir:${ELIXIR_VERSION}-erlang-${OTP_VERSION}-debian-${DEBIAN_VERSION}"
ARG RUNNER_IMAGE="docker.io/debian:${DEBIAN_VERSION}"

FROM ${BUILDER_IMAGE} AS builder

# install build dependencies (including Node.js for React build)
RUN apt-get update \
  && apt-get install -y --no-install-recommends build-essential git curl \
  && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
  && apt-get install -y nodejs \
  && rm -rf /var/lib/apt/lists/*

# prepare build dir
WORKDIR /app

# install hex + rebar
RUN mix local.hex --force \
  && mix local.rebar --force

# set build ENV
ENV MIX_ENV="prod"

# install mix dependencies
COPY katanaute/mix.exs katanaute/mix.lock ./
RUN mix deps.get --only $MIX_ENV
RUN mkdir config

# copy compile-time config files before we compile dependencies
# to ensure any relevant config change will trigger the dependencies
# to be re-compiled.
COPY katanaute/config/config.exs katanaute/config/${MIX_ENV}.exs config/
RUN mix deps.compile

RUN mix assets.setup

COPY katanaute/priv priv

# Copy Mix task for React build
COPY katanaute/lib/mix lib/mix

COPY katanaute/lib lib

# Compile the release
RUN mix compile

# Copy React source to ../katareact so the mix task can find it
RUN mkdir -p ../katareact
COPY katareact ../katareact

# Build React frontend using the Mix task
RUN mix react.build

COPY katanaute/assets assets

# compile assets
RUN mix assets.deploy

# Changes to config/runtime.exs don't require recompiling the code
COPY katanaute/config/runtime.exs config/

COPY katanaute/rel rel
RUN mix release

# start a new build stage so that the final image will only contain
# the compiled release and other runtime necessities
FROM ${RUNNER_IMAGE} AS final

RUN apt-get update \
  && apt-get install -y --no-install-recommends libstdc++6 openssl libncurses5 locales ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Set the locale
RUN sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen \
  && locale-gen

ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US:en
ENV LC_ALL=en_US.UTF-8

WORKDIR "/app"
RUN chown nobody /app

# set runner ENV
ENV MIX_ENV="prod"

# Only copy the final release from the build stage
COPY --from=builder --chown=nobody:root /app/_build/${MIX_ENV}/rel/katanaute ./

USER nobody

EXPOSE 80

# If using an environment that doesn't automatically reap zombie processes, it is
# advised to add an init process such as tini via `apt-get install`
# above and adding an entrypoint. See https://github.com/krallin/tini for details
# ENTRYPOINT ["/tini", "--"]

CMD ["/app/bin/server"]
