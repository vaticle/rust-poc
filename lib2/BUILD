load("@rules_rust//rust:defs.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "assemble_crate", "deploy_crate")
load("@vaticle_dependencies//distribution:deployment.bzl", "deployment")


package(default_visibility = ["//visibility:public"])

rust_library(
    name = "lib2",
    srcs = ["lib.rs"],
    deps = [
        "@raze__better_any__0_1_1//:better_any",
    ],
    tags = ["crate-name=lllllxxxxx"]
)

assemble_crate(
    name = "assemble-crate",
    target = ":lib2",
    description = "TypeQL Grammar for Rust",
    license = "AGPL-3.0-only",
    readme_file = "//:README.md",
    homepage = "https://github.com/vaticle/typeql",
    repository = "https://github.com/vaticle/typeql",
    keywords = ["grakn", "database", "graph", "knowledgebase", "knowledgeengineering"],
    authors = ["Vaticle <community@vaticle.com>"]
)

deploy_crate(
    name = "deploy-crate",
    target = ":assemble-crate",
    snapshot = deployment["crate.snapshot"],
    release = deployment["crate.release"],
)