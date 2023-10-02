load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "db89135f4d1eaa047b9f5518ba4037284b43fc87386d08c1d1fe91708e3730ae",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.27.0/rules_rust-v0.27.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.72.0"
    ],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate")
load("@rules_rust//crate_universe:defs.bzl", "splicing_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo-bazel-lock.json",
    splicing_config = splicing_config("2"),
    # TODO: Mess around, trying to get openssl to build with the "vendored" feature.
    annotations = {
        # "openssl-src": [crate.annotation(
        #     data = ["openssl"],
        # )],
        # "openssl-sys": [crate.annotation(
        #     data = ["openssl"],
        # )],
    },
    manifests = [
        "//:Cargo.toml",
        "//:scratch/Cargo.toml",
        "//:scratch-bin/Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
