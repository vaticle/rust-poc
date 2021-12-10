load("@rules_rust//rust:rust.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "rust_pkg_intellij")

srcs = glob([
    "lib2.rs",
    "veg/*.rs",
])

rust_library(
    name = "lib2",
    srcs = srcs,
    visibility = ["//visibility:public"]
)

rust_pkg_intellij(
    name = "pkg",
    target = ":lib2",
    srcs = srcs,
    description = "Hello and goodbye (2)",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
