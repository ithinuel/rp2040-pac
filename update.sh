#!/usr/bin/env bash

set -ex

cargo install --version 0.28.0 svd2rust
cargo install --version 0.10.0 form
#cargo install --version 0.3.0  svdtools
rustup component add rustfmt

rm -rf src
mkdir src

svdtools patch svd/rp2040.yaml
mv svd/rp2040.svd.patched svd/rp2040-pass-1.svd
svdtools patch svd/rp2040-pass-2.yaml
mv svd/rp2040-pass-1.svd.patched svd/rp2040.svd.patched

svd2rust -i svd/rp2040.svd.patched

form -i lib.rs -o src
rm lib.rs

cargo fmt

# Original svd has \n (two chars) in it, which gets converted to "\n" by svd2rust
# If we convert them to newline characters in the SVD, they don't turn up in markdown so docs suffers
# So, convert \n to [spc] [spc] [newline], then strip the spaces out if there are consecutive [newlines]
# This means that by the time we're in markdown \n\n becomes new paragraph, and \n becomes a new line
if [ "$(uname)" == "Darwin" ]; then
    find src -name '*.rs' -exec sed -i '' -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
else
    find src -name '*.rs' -exec sed -i -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
fi

# Sort specified fields alphanumerically for easier consumption in docs.rs
./sortFieldsAlphaNum.sh
