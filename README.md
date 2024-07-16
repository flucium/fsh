# FSH

...

## Build

<details>
<summary>Linux</summary>

```bash
# tools
apt update -y &&\
apt install -y git build-essential &&\
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source "$HOME/.cargo/env" && \
rustup toolchain add 1.77.2 && \
rustup default 1.77.2 && \
```

```bash
# Debug build
git clone git@github.com:flucium/fsh.git
zsh ./fsh/build.sh debug
```

```bash
# Release build
git clone git@github.com:flucium/fsh.git
zsh ./fsh/build.sh release
```

</details>

<!-- <br> -->

<!-- <details>
<summary>macOS</summary>
...
</details>

<br> -->

<!-- <details>
<summary>Windows</summary>
...
</details>

<br> -->

<!-- <details>
<summary>Cross compile</summary>
...
</details> -->

## Installation
...

## How to use
...

<!-- ## Prototype
Repository<br>
[https://github.com/flucium/flatshell](https://github.com/flucium/flatshell)

Development Essay (rough sketch)<br>
[https://github.com/flucium/flatshell/blob/main/fsh_development/fsh.pdf](https://github.com/flucium/flatshell/blob/main/fsh_development/fsh.pdf) -->

## License
FSH is licensed under the MIT License. For third-party libralies and other dependencies, please refer to LICENSE_THIRDPARTY.md.

[FSH License](./LICENSE)

[THIRDPARTY License](./LICENSE_THIRDPARTY)

## Sponsors
...