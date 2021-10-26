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

