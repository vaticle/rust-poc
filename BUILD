load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "rust_cargo_binary")

rust_binary(
    name = "main",
    srcs = [
        "main.rs",
    ],
    deps = [
        "//lib1:lib1",
        "//lib2:lib2",
    ]
)

rust_cargo_binary(
    name = "pkg",
    target = ":main",
    description = "Hello, hello",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
