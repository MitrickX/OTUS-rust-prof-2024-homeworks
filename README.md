cargo clippy --tests && cargo clippy --all-targets --all && cargo fmt --check

zip -r hw4.zip hw4 -x "hw4/target/*"
