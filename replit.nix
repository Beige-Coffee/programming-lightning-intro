{ pkgs }: {
	deps = [
   pkgs.netcat-openbsd
   pkgs.rustup
    pkgs.electrs
    pkgs.bitcoin
		pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
    pkgs.rust-analyzer
		pkgs.openssl
		pkgs.pkg-config
		pkgs.libffi
	];
}