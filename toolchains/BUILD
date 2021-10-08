# Bazel's platform/toolchain APIs require this wrapper around the actual
# toolchain defined above. It serves two purposes: declare which
# constraint_values it supports (which can be matched to appropriate platforms)
# and tell Bazel what language this toolchain is for.
#
# So when you're building a cc_binary, Bazel has all the info it needs to give
# that cc_binary the right toolchain: it knows cc_binary requires a "C++-type
# toolchain" (this is encoded in the cc_binary rule definition) and needs to
# use a toolchain that matches whatever you set --platforms to at the command
# line.
"""
toolchain(
    name = "macos_musl_toolchain",
    exec_compatible_with = [
        "@platforms//cpu:x86_64",
        "@platforms//os:macos",
    ],
    target_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
        "//build/platform/libc:musl",
    ],
    toolchain = ":macos_musl_cc_toolchain",
    toolchain_type = "@bazel_tools//tools/cpp:toolchain_type",
)
"""