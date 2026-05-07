//@ build-pass (FIXME(55996): should be run on targets supporting avx)
//@ ignore-test
//@ no-prefer-dynamic
//@ compile-flags: -Ctarget-feature=+avx -Clto

fn main() {}
