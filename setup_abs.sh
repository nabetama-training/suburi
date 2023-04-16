echo "Enter the name of the new binary: "
read bin_name

echo "--- Defining $bin_name at Cargo.toml ---"
cat << EOT >> Cargo.toml

[[bin]]
name = "$bin_name"
path = "src/bin/abs/$bin_name.rs"
EOT

echo "--- Creating src/bin/abs/$bin_name.rs ---"
cp src/bin/abs/template.rs src/bin/abs/$bin_name.rs
# `gsed` means GNU sed, which is installed by `brew install gnu-sed
gsed -i -e "s/<BIN_NAME>/$bin_name/g" src/bin/abs/$bin_name.rs

echo "--- Done! ---"