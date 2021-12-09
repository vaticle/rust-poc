#
# Copyright (C) 2021 Vaticle
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@vaticle_bazel_distribution//crates:rules.bzl", "assemble_crate", "deploy_crate")
load("//:deployment.bzl", "deployment")


rust_binary(
    name = "main",
    srcs = ["main.rs"],
    deps = [
        "//lib1:lib1",
    ]
)

assemble_crate(
    name = "assemble",
    target = ":main",
    description = "Hello, hello",
    homepage = "https://github.com/vaticle/rust-poc",
    license = "MIT",
    repository = "https://github.com/vaticle/rust-poc",
)
