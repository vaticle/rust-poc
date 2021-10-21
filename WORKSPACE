load("//dependencies/vaticle:repositories.bzl", "vaticle_dependencies")
vaticle_dependencies()

# Load //builder/rust
load("@vaticle_dependencies//builder/rust:deps.bzl", rust_deps = "deps")
rust_deps()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")
#rust_repositories()
rust_repositories(version = "nightly", iso_date = "2021-07-01", edition="2018")

load("@vaticle_dependencies//library/crates:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()


load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "or_tools_linux",
    urls = ["https://github.com/google/or-tools/releases/download/v9.0/or-tools_ubuntu-20.10_v9.0.9048.tar.gz"],
    strip_prefix = "or-tools_Ubuntu-20.10-64bit_v9.0.9048/",
    sha256 = "1cdf059512062f085a18d37dd0ffe4511cd3a2342dab189c3f255fd94fb2a610",
    build_file_content =
"""
cc_import(
    name = "lib",
    shared_library = "lib/libortools.so",
    visibility = ["//visibility:public"]
)
cc_library(
   name = "incl",
   hdrs = glob([
       "include/ortools/**/*.h",
       "include/absl/**/*.h",
       "include/absl/**/*.inc",
       "include/google/protobuf/**/*.inc",
       "include/google/protobuf/**/*.h",
   ]),
   strip_include_prefix = "include/",
   visibility = ["//visibility:public"]
)
"""
)

http_archive(
        name = "or_tools_mac",
        urls = ["https://github.com/google/or-tools/releases/download/v9.0/or-tools_MacOsX-11.2.3_v9.0.9048.tar.gz"],
        strip_prefix = "or-tools_MacOsX-11.2.3_v9.0.9048/",
        sha256 = "adf73a00d4ec49558b67be5ce3cfc8f30268da2253b35feb11d0d40700550bf6",
        build_file_content =
"""
cc_import(
    name = "lib",
    shared_library = "lib/libortools.dylib",
    visibility = ["//visibility:public"]
)
cc_library(
   name = "incl",
   hdrs = glob([
       "include/ortools/**/*.h",
       "include/absl/**/*.h",
       "include/absl/**/*.inc",
       "include/google/protobuf/**/*.inc",
       "include/google/protobuf/**/*.h",
   ]),
   strip_include_prefix = "include/",
   visibility = ["//visibility:public"]
)
"""
)

http_archive(
        name = "or_tools_windows",
        urls = ["https://github.com/google/or-tools/releases/download/v9.0/or-tools_VisualStudio2019-64bit_v9.0.9048.zip"],
        strip_prefix = "or-tools_VisualStudio2019-64bit_v9.0.9048/",
        sha256 = "1be7286e082ba346f8729a873c5fd85418ac2dc95b847d9baa5381c5ac5f5fd9",
        build_file_content =
"""
cc_import(
    name = "lib",
    shared_library = "lib/ortools.lib",
    visibility = ["//visibility:public"]
)
cc_library(
   name = "incl",
   hdrs = glob([
       "include/ortools/**/*.h",
       "include/absl/**/*.h",
       "include/absl/**/*.inc",
       "include/google/protobuf/**/*.inc",
       "include/google/protobuf/**/*.h",
   ]),
   strip_include_prefix = "include/",
   visibility = ["//visibility:public"]
)
"""
)


# Load //builder/python
load("@vaticle_dependencies//builder/python:deps.bzl", python_deps = "deps")
python_deps()

# Load //builder/java
load("@vaticle_dependencies//builder/java:deps.bzl", java_deps = "deps")
java_deps()

# Load //builder/kotlin
load("@vaticle_dependencies//builder/kotlin:deps.bzl", kotlin_deps = "deps")
kotlin_deps()
load("@io_bazel_rules_kotlin//kotlin:kotlin.bzl", "kotlin_repositories", "kt_register_toolchains")
kotlin_repositories()
kt_register_toolchains()

load("@vaticle_dependencies//tool/common:deps.bzl", vaticle_dependencies_tool_maven_artifacts = "maven_artifacts")

# Load //builder/antlr
load("@vaticle_dependencies//builder/antlr:deps.bzl", antlr_deps = "deps", "antlr_version")
antlr_deps()

load("@rules_antlr//antlr:lang.bzl", "JAVA", "RUST")
load("@rules_antlr//antlr:repositories.bzl", "rules_antlr_dependencies")
rules_antlr_dependencies("4.8.2-rust", JAVA, RUST)

load("@vaticle_dependencies//library/maven:rules.bzl", "maven")
maven(vaticle_dependencies_tool_maven_artifacts)
