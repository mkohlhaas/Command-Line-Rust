#!/usr/bin/env bash

for d in $(find -name Cargo.toml); do
  pushd .
  cd $(dirname $d)
  cargo clippy
  popd
done

echo "Done."
