load("@rules_rust//rust:rust.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "rust_cargo_library")

rust_library(
    name = "lib2",
    srcs = ["lib.rs"],
    visibility = ["//visibility:public"]
)

rust_cargo_library(
    name = "pkg",
    target = ":lib2",
    description = "Hello, hello",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
