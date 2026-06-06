load("@rules_foreign_cc//foreign_cc:defs.bzl", "cmake")

filegroup(
    name = "all_srcs",
    srcs = glob(["**"]),
)

cmake(
    name = "fast_float",
    generate_args = ["-GNinja"],
    includes = ["include"],
    install = True,
    lib_source = ":all_srcs",
    out_headers_only = True,
    visibility = ["//visibility:public"],
)

# filegroup(
#     name = "gen_dir",
#     srcs = [":fast_float"],
#     output_group = "gen_dir",
#     visibility = ["//visibility:public"],
# )
