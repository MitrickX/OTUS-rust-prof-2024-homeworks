cargo clippy --tests && cargo clippy && cargo fmt --check

zip -r hw4.zip hw4 -x "hw4/target/*"
