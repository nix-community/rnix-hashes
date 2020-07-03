{ rustChannelOf }:
 (rustChannelOf {
      date = "2020-06-30";
      channel = "nightly";
      sha256 = "g6FUfiAPc86qovuDviwf35EgOlA6lQKIcO70g3Oz3R4=";
}).rust.override {
  extensions = [
    "clippy-preview"
    "rls-preview"
    "rustfmt-preview"
    "rust-analysis"
    "rust-std"
    "rust-src"
  ];
  targets = [ "wasm32-unknown-unknown" ];
}
