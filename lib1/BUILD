load("@rules_rust//rust:rust.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "rust_pkg_intellij")

srcs = glob([
    "lib1.rs",
    "fruit/*.rs",
])

rust_library(
    name = "lib1",
    srcs = srcs,
    visibility = ["//visibility:public"]
)

rust_pkg_intellij(
    name = "pkg",
    target = ":lib1",
    srcs = srcs,
    lib = "lib1.rs",
    description = "Hello and goodbye (1)",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
