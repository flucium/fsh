
list()
{
    echo "Target OS List"
    echo "- ubuntu: Ubuntu 20.04 LTS"
}

ubuntu()
{
    apt update && \
    apt install -y curl git vim git build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
    source "$HOME/.cargo/env" && \
    rustup toolchain add 1.77.2 && \
    rustup default 1.77.2 && \
    git clone git@github.com:flucium/fsh.git ~/repos && \
    mkdir ~/repos/fsh/temp && \
    echo "$FSH_PROMPT = '$ '" > ~/repos/fsh/temp/profile.fsh && \
    ls ~/repos/fsh
}


for entry in "$@" ; do
  r=$?
  if [ "x$r" = "x0" ]; then
    $entry
  fi
done
