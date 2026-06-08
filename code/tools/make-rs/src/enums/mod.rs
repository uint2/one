#[derive(Debug, Clone, Copy)]
pub enum Compiler {
    Gcc,
    Clang,
    Tcc,
    Msvc,
}

impl Compiler {
    pub const fn as_naive_str(&self) -> &'static str {
        match self {
            Self::Gcc => "/usr/bin/cc",
            _ => todo!(),
        }
    }
}

/// Referenced from CMake specification:
/// https://cmake.org/cmake/help/latest/prop_tgt/C_STANDARD.html
#[derive(Debug, Clone, Copy)]
pub enum CStandard {
    /// C89/C90
    C89,
    C90,
    C99,
    C11,
    C17,
    C23,
}

/// Referenced from CMake specification:
/// https://cmake.org/cmake/help/latest/prop_tgt/CXX_STANDARD.html
#[derive(Debug, Clone, Copy)]
pub enum CxxStandard {
    Cxx98,
    Cxx11,
    Cxx14,
    Cxx17,
    Cxx20,
    Cxx23,
    Cxx26,
}

#[derive(Debug, Clone, Copy)]
pub enum LanguageStandard {
    C(CStandard),
    Cxx(CxxStandard),
}

/// In C/C++, this would be `#define KEY VALUE`
pub struct CompileDefinition {
    key: &'static str,
    value: Option<&'static str>,
}

impl CompileDefinition {
    pub fn of(key: &'static str) -> Self {
        Self { key, value: None }
    }

    pub fn kv(key: &'static str, value: &'static str) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }

    pub fn to_c_flag(&self) -> String {
        match self.value {
            Some(ref value) => format!("-D{}={value}", self.key),
            None => format!("-D{}", self.key),
        }
    }
}
