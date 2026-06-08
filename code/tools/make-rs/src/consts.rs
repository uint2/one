/*
<workspace root>
├── build
│   ├── deps        # Everything related to external dependencies.
│   │   ├── src     # External source files.
│   │   └── build   # External build outputs.
│   └── ...
├── <workspace files>
└── ...
*/

#[allow(unused)]
pub mod dir {
    pub const BUILD: &'static str = "build";
    pub mod build {
        pub const DEPS: &'static str = "build/deps";
        pub mod deps {
            /// Source directory for external dependencies. Clone their repos here.
            pub const SOURCE: &'static str = "build/deps/src";
            /// Build directory for external dependencies.
            pub const BUILD: &'static str = "build/deps/build";
        }
    }
}
