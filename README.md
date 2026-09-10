# purescript-arrays

[![Latest release](http://img.shields.io/github/release/purescript/purescript-arrays.svg)](https://github.com/purescript/purescript-arrays/releases)
[![Build status](https://github.com/purescript/purescript-arrays/workflows/CI/badge.svg?branch=master)](https://github.com/purescript/purescript-arrays/actions?query=workflow%3ACI+branch%3Amaster)
[![Pursuit](https://pursuit.purescript.org/packages/purescript-arrays/badge)](https://pursuit.purescript.org/packages/purescript-arrays)

Utility functions for the `Array` type - JavaScript's native arrays.

## Installation

```
spago install arrays
```

## Documentation

Module documentation is [published on Pursuit](http://pursuit.purescript.org/packages/purescript-arrays).

## Rust tests

Run `bin/test -c` to rebuild the sibling `purust` compiler, clear this package's
caches and generate fresh TAST and Rust. `bin/test` skips the compiler rebuild.
Both use the compiler's local Spago dependency and select the sibling TAST-enabled
PureScript fork; `PURS=/path/to/purs` overrides that selection.

`Test.Main` and all five original test modules are preserved: Array, mutable
Array/ST, both partial modules, and NonEmptyArray. They retain their original
assertions, including the 50,000-element `replicateA` stack-safety checks.
`Test.Runner` adds six groups for captured callbacks and records across an
opaque FFI boundary, immutable snapshots, independent ST allocations, replayed
ST actions, checked bounds, Iterator operations, and deferred/replayed Effect
`replicateA` up to 100,000 elements.

Three Rust tests call the FFI directly to check callback order and short-circuit
behavior, stable sorting, bounds, input preservation, ST snapshots and repeated
operations. The Bash runner checks exit status, complete stdout and empty stderr,
with a 60-second timeout for the PureScript suite. No JavaScript test runner file
is needed.
