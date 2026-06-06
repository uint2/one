# Paper

A cross-device drawing app that is lightweight and transparent.
Fast buffer-switching and fuzzing finding documents.

## Feature Goals

- Inter-document references
- Expose files to users, even if it's just binary (allows for git
  committing)
- Color scheme support (with h1-h6, etc.)

## Architecture choice

#### Drawing engine

1.  ✅ Self-implemented
2.  Use Apple's `PencilKit` library

> `PencilKit` does not support Mac OS as of 05/05/2023

#### UI framework

1. ✅ SwiftUI
2. Cocoa

> SwiftUI because Mac Catalyst supports this better
