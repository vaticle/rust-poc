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
        "@vaticle_dependencies//util/platform:is_windows": [
           "@or_tools_windows//:lib",
           "@or_tools_windows//:incl",
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
        "@vaticle_dependencies//util/platform:is_windows": [
           "@or_tools_windows//:lib",
           "@or_tools_windows//:incl",
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
    ]
)
