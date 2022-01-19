load("@rules_rust//rust:defs.bzl", "rust_library")

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
