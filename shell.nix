{ mkShell, pkgsCross, fenix  }:

let
  toolchain = fenix.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
  };
in

mkShell {
  packages = [ toolchain ];
}
