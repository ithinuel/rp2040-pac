#!/usr/bin/env bash

# Path to `svd`/`svdtools`
SVDTOOLS="${SVDTOOLS:-svdtools}"
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

set -exuo pipefail

cargo install --version 0.36.0 svd2rust --locked
cargo install --version 0.12.1 form --locked
rustup component add rustfmt
if [ "$SVDTOOLS" == "svdtools" ]; then
    cargo install --version 0.3.20 svdtools --locked
else
    python3 -mvenv --clear .venv
    source .venv/bin/activate
    pip3 install --upgrade "svdtools==0.1.25"
fi

$SVDTOOLS patch svd/rp2040.yaml
perl -0777 -pi -e 's{(<description>.*?</description>)}{
    my $x = $1;
    $x =~ s/\n\n/\\n\\n\n/g; # merge dual newlines in their escaped version
    $x =~ s/([^\\][^n])\n/\1\\n\n/g; # escape newlines that are not already escaped
    $x =~ s/\\n\n\\n\n/\\n\\n\n/g;   # merge escaped newline sequences
    $x;
}ges' svd/rp2040.svd.patched

if [ "$SVDTOOLS" != "svdtools" ]; then
    deactivate
fi

generate() {
    local svd=$1
    local target=$2
    svd2rust -i "$svd" -c "${SCRIPT_DIR}/svd2rust.toml" --target "$target"
    form -i "$3.rs" -o src
}

# Most of the code is from Cortex-M mode
tmp_dir=$(mktemp -d -t svd2rust-XXXX)
pushd "${tmp_dir}"
generate "${SCRIPT_DIR}/svd/rp2040.svd.patched" cortex-m lib

rm -rf "${SCRIPT_DIR}/src"
mv {src,device.x} "${SCRIPT_DIR}"

popd
rm -rf "${tmp_dir}"

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
./sortFieldsAlphaNum.sh src/lib.rs
