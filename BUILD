load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "rust_pkg_intellij")

srcs = ["main.rs"]

rust_binary(
    name = "main",
    srcs = srcs,
    deps = [
        "//lib1:lib1",
        "//lib2:lib2",
    ]
)

rust_pkg_intellij(
    name = "pkg",
    target = ":main",
    srcs = srcs,
    bin = "main.rs",
    description = "Hello and goodbye repeatedly",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
