#!/usr/bin/env bash

for d in $(find -name Cargo.toml); do
  pushd .
  cd $(dirname $d)
  cargo fmt
  popd
done

echo "Done."
