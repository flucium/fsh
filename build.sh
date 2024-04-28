release()
{
  cargo build --release
  mkdir ./fsh-0.0.1-aarch64-linux
  cp -r ./target/release/* ./fsh-0.0.1-aarch64-linux/
  cp ./LICENSE ./fsh-0.0.1-aarch64-linux/LICENSE
  cp ./README.md ./fsn-0.0.1-aarch64-linux/README.md
  tar -zcvf fsh-0.0.1-aarch64-linux.tar.gz ./fsh-0.0.1-aarch64-linux
  rm -r ./fsh-0.0.1-aarch64-linux
}

debug()
{
  cargo build
}

clean()
{
  cargo clean
  if [ -f ./fsh-0.0.1-aarch64-linux.tar.gz ]; then
    rm ./fsh-0.0.1-aarch64-linux.tar.gz
  fi
}

for entry in "$@" ; do
  r=$?
  if [ "x$r" = "x0" ]; then
    $entry
  fi
done
