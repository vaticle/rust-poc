load("//dependencies/vaticle:repositories.bzl", "vaticle_dependencies")
vaticle_dependencies()

load("@vaticle_dependencies//distribution:deps.bzl", "vaticle_bazel_distribution")
vaticle_bazel_distribution()


# Load //builder/rust
load("@vaticle_dependencies//builder/rust:deps.bzl", rust_deps = "deps")
rust_deps()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")
#rust_repositories()
rust_repositories(version = "nightly", iso_date = "2021-07-01", edition="2018")

load("@vaticle_dependencies//library/crates:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()


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
