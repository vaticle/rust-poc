load("@rules_rust//rust:defs.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "assemble_crate", "deploy_crate")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "lib3",
    srcs = ["lib.rs"],
    deps = [],
    tags = ["crate-name=CUSTOMLIB"]
)

assemble_crate(
    name = "assemble-crate",
    target = ":lib3",
    description = "TypeQL Grammar for Rust",
    license = "AGPLv3",
    readme_file = "//:README.md",
    homepage = "https://github.com/vaticle/typeql",
    repository = "https://github.com/vaticle/typeql",
    keywords = ["grakn", "database", "graph", "knowledgebase", "knowledgeengineering"],
    authors = ["Vaticle <community@vaticle.com>"]
)
