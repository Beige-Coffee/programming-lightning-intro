{ pkgs }: {
	deps = [
    pkgs.electrs
    pkgs.bitcoin
		pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}