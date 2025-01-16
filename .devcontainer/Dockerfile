FROM ubuntu:noble
ARG USERNAME=dev
ARG REPO=/ws

ENV PATH="/home/$USERNAME/.local/bin:/home/$USERNAME/.cargo/bin:$PATH"

ARG DEBIAN_FRONTEND=noninteractive
RUN set -eux; \
    apt update; \
    apt install -y ca-certificates libc6-dev pkg-config libssl-dev clang zsh sudo git wget

RUN set -eux; \
    grep ubuntu /etc/passwd && \
    touch /var/mail/ubuntu && \
    chown ubuntu /var/mail/ubuntu && \
    userdel -r ubuntu

RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$USERNAME"; \
    echo "$USERNAME ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/$USERNAME"; \
    chmod 0440 "/etc/sudoers.d/$USERNAME"

RUN set -eux; \
    mkdir -p "$REPO"; \
    chown -R "$USERNAME:$USERNAME" "$REPO" "/home/$USERNAME"

WORKDIR "$REPO"
USER "$USERNAME"

RUN set -eux; \
    wget -qO - https://sh.rustup.rs | sh -s -- -y

CMD sleep infinity
