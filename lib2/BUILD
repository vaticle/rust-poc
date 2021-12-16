load("@rules_rust//rust:rust.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "rust_pkg_intellij")

srcs = glob([
    "lib2.rs",
    "veg/*.rs",
])

rust_library(
    name = "lib2",
    srcs = srcs,
    deps = [
        "@vaticle_dependencies//library/crates:rand",
    ],
    visibility = ["//visibility:public"]
)

rust_pkg_intellij(
    name = "pkg",
    target = ":lib2",
    srcs = srcs,
    lib = "lib2.rs",
    description = "Hello and goodbye (2)",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
