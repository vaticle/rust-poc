load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")
"""
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
    ]
)

"""

rust_binary(
    name = "hello",
    srcs = [
        "hello.rs",
    ],
    deps = [
#        "@vaticle_dependencies//library/crates:rocksdb",
#        "@vaticle_dependencies//library/crates:cxx",
#        ":bridge",
    ]
)

platform(
    name = "MACC",
    constraint_values = [
        "@platforms//os:windows",
        "@platforms//cpu:x86_64",
    ],
)

package(default_visibility = ["//visibility:public"])

cc_toolchain_suite(
    name = "mingw_suite",
    toolchains = {
        "k8": ":k8_toolchain",
    },
)



filegroup(name = "empty")

cc_toolchain(
    name = "k8_toolchain",
    toolchain_identifier = "k8-toolchain",
    toolchain_config = ":k8_toolchain_config",
    all_files = ":empty",
    compiler_files = ":empty",
    dwp_files = ":empty",
    linker_files = ":empty",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 0,
)

load(":cc_toolchain_config.bzl", "cc_toolchain_config")
cc_toolchain_config(name = "k8_toolchain_config")