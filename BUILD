load("@vaticle_dependencies//builder/antlr:rules.bzl", "python_grammar_adapter")
load("@rules_antlr//antlr:antlr4.bzl", "antlr")

load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

load("@vaticle_dependencies//builder/rust:rules.bzl", "rust_cxx_bridge")

cc_library(
    name = "bridge-wrapper",
    hdrs = ["OrToolsWrapper.h"],
    deps = select({
       "@vaticle_dependencies//util/platform:is_mac": [
           "@or_tools_mac//:lib",
           "@or_tools_mac//:incl",
       ],
       "@vaticle_dependencies//util/platform:is_linux": [
           "@or_tools_linux//:lib",
           "@or_tools_linux//:incl",
        ],
    })
)

rust_cxx_bridge(
    name = "bridge",
    src = "ortools.rs",
    deps = select({
       "@vaticle_dependencies//util/platform:is_mac": [
           "@or_tools_mac//:lib",
           "@or_tools_mac//:incl",
       ],
       "@vaticle_dependencies//util/platform:is_linux": [
           "@or_tools_linux//:lib",
           "@or_tools_linux//:incl",
        ],
    }) + [":bridge-wrapper"],
)

rust_binary(
    name = "main",
    srcs = [
        "main.rs",
        "ortools.rs",
    ],
    deps = [
        "@vaticle_dependencies//library/crates:rocksdb",
        "@vaticle_dependencies//library/crates:cxx",
        ":bridge",
        ":typeqlgrammar",
    ]
)

python_grammar_adapter(
    name = "rust-grammar",
    input = "TypeQL.g4",
    output = "TypeQLRust.g4",
)

antlr(
    name = "typeqlgrammar-src",
    srcs = [":rust-grammar"],
    language = "Rust",
    visitor = True,
    package = "typeqlgrammar",
)


rust_library(
    name = "typeqlgrammar",
    srcs = [":typeqlgrammar-src"],
    deps = [
        "@vaticle_dependencies//library/crates:antlr_rust",
    ]
)
