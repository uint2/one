load("@rules_foreign_cc//foreign_cc:defs.bzl", "cmake")

filegroup(
    name = "all_srcs",
    srcs = glob(["**"]),
)

cmake(
    name = "neovim-deps",
    generate_args = ["-GNinja"],
    lib_source = ":all_srcs",
    out_shared_libs = [],
    tags = ["requires-network"],
    visibility = ["//visibility:public"],
    working_directory = "cmake.deps",
)

cmake(
    name = "neovim",
    generate_args = ["-GNinja"],
    lib_source = ":all_srcs",
    visibility = ["//visibility:public"],
    deps = [":neovim-deps"],
)
