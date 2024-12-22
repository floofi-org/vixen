{ mkShell, pkgsCross, fenix  }:

let
  toolchain = fenix.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
  };
in

mkShell {
  packages = [ toolchain ];
}
