load("//dependencies/vaticle:repositories.bzl", "vaticle_dependencies")
vaticle_dependencies()
#local_repository(
#    name = "vaticle_dependencies",
#    path = "/home/vmax/work/vaticle/dependencies",
#)

# Load //builder/rust
load("@vaticle_dependencies//builder/rust:deps.bzl", rust_deps = "deps")
rust_deps()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

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
    static_library = "lib/ortools.lib",
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
