load("@rules_rust//rust:defs.bzl", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "assemble_crate", "deploy_crate")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "lib1",
    srcs = ["lib.rs"],
    deps = [
        "//lib2:lib2",
        "//lib3:lib3",
    ],
)

assemble_crate(
    name = "assemble-crate",
    target = ":lib1",
    description = "TypeQL Grammar for Rust",
    license = "AGPLv3",
    readme_file = "//:README.md",
    homepage = "https://github.com/vaticle/typeql",
    repository = "https://github.com/vaticle/typeql",
    keywords = ["grakn", "database", "graph", "knowledgebase", "knowledgeengineering"],
    authors = ["Vaticle <community@vaticle.com>"]
)
