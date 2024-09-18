set -xe

terraform init -input=false
cd ..
cargo lambda build --release --output-format zip