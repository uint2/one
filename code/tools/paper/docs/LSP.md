# Attempts to use sourcekit-lsp alongside this Xcode project

### Using a dummy Package.swift to allow `swift build` to work

```swift
// swift-tools-version: 5.8
import PackageDescription

let APP_NAME = "Paper"

let package = Package(
    name: APP_NAME,
    defaultLocalization: "en",
    platforms: [.iOS(.v16), .macOS(.v13)],
    dependencies: [],
    targets: [
        .executableTarget(
            name: APP_NAME,
            dependencies: [],
            path: APP_NAME
            // resources: [.process("Resources")]
        )
    ])
```

Couldn't work because even if swift could find the IOS SDK after given
additional flags, sourcekit-lsp couldn't pick up on the build index.

### Using OTHER_CFLAGS to pass `-gen-cdb-fragment-path` to `xcodebuild`

Xcode refuses to actually pass that flag on to `clang`. Made obvious
by passing bad flags and the build still going smoothly.
