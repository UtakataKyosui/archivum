FROM mcr.microsoft.com/devcontainers/base:ubuntu

ARG USERNAME=utakata
ARG USER_UID=1100
ARG USER_GID=${USER_UID}

RUN groupadd --gid ${USER_GID} ${USERNAME} \
     && useradd -m -s /bin/bash --uid ${USER_UID} --gid ${USER_GID} ${USERNAME} \
     && apt-get update \
     && apt-get install -y sudo curl xz-utils ca-certificates build-essential \
     && echo "${USERNAME} ALL=(root) NOPASSWD:ALL" >/etc/sudoers.d/${USERNAME} \
     && chmod 0440 /etc/sudoers.d/${USERNAME}

USER ${USERNAME}

RUN curl -L https://releases.nixos.org/nix/nix-2.30.1/install | sh -s -- --no-daemon

ENV PATH="/home/${USERNAME}/.nix-profile/bin:${PATH}"

RUN mkdir -p ~/.config/nix \
     && printf "experimental-features = nix-command flakes\n" \
     > ~/.config/nix/nix.conf

