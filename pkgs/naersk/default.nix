{ naersk, rust }:
(naersk {
  rustc = rust.rust;
  cargo = rust.rust;
})
