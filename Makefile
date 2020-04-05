##
# roguelike-rs
#
# @file
# @version 0.1

run:
	nix-shell --run "cargo run --release"

clippy: 
	nix-shell --run "cargo clippy --release"

test:
	nix-shell --run "cargo test"
