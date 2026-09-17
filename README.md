# mold-macho: mold for macOS

mold-macho is a Mach-O linker for macOS, written in Rust as a port of
the [mold linker](https://github.com/rui314/mold) to Apple platforms.
It is a drop-in replacement for Apple's `ld` for common workloads: it
accepts ld64's command line, and links working, correctly code-signed
executables, dylibs and bundles for arm64 and x86-64 macOS.

C, C++, Objective-C, Swift and Rust programs link and run, including
real-world projects (ripgrep builds and passes its full test suite;
the C++ mold tree with vendored tbb/zstd/zlib builds at -O3; the
linker links itself). Debugging works (stabs for lldb/dsymutil), and
so do C++ exceptions, thread-locals, LTO, auto-linking, frameworks and
Objective-C selector stubs.

## Usage

Build with `cargo build --release`. The binary is `target/release/mold`;
`./install.sh` (PREFIX=/usr/local by default) installs it as
`mold` with an `ld64.mold` symlink, the same arrangement as mold's
`ld.mold`. Then point your compiler driver at the linker:

    clang -o hello hello.c --ld-path=path/to/ld64.mold
    swiftc -o hello hello.swift -use-ld=path/to/ld64.mold
    RUSTFLAGS="-C link-arg=--ld-path=path/to/ld64.mold" cargo build

## Binary releases

Pushes to `main` build native Apple Silicon and Intel binaries on macOS 26,
targeting macOS 15 or later, with fat LTO and one codegen unit. Tests run on
macOS 26; macOS 15 is not tested separately. New pushes cancel superseded
builds.

Each completed build publishes an immutable release tagged
`sha-<full-commit-SHA>`, with `mold-macho-aarch64-apple-darwin.tar.gz`,
`mold-macho-x86_64-apple-darwin.tar.gz`, and `SHA256SUMS`. Each archive
contains `bin/mold`, a `bin/ld64.mold` symlink, and `LICENSE`. Each release
binary is built and tested with only its matching architecture enabled.
Local builds still enable both architectures by default. To verify both
downloaded archives:

    shasum -a 256 -c SHA256SUMS

Extract the archive for your Mac and point your compiler at its
`bin/ld64.mold`, or copy both entries in `bin/` to a directory on your PATH.

The release workflow requires **Settings > General > Releases > Enable
release immutability** to be enabled in GitHub. It creates a draft release,
uploads both archives and checksums, then publishes the release to lock its
assets. Reruns can resume incomplete draft releases but never replace
published releases.

## Feature highlights

- Classic dyld info and chained fixups (the default for deployment
  targets of macOS 13+), ad-hoc code signing, export tries, and
  content-hash UUIDs
- Subsections-via-symbols atoms, `-dead_strip`, identical code folding
  (on by default, like ld64's deduplication), literal merging
- `__unwind_info` synthesis from compact unwind, `__eh_frame`
  re-synthesis for DWARF-only unwind, range-extension thunks
- Archives with mold's order-insensitive resolution model, .tbd stubs
  with umbrella reexports, frameworks, auto-linking
  (LC_LINKER_OPTION), LTO via libLTO
- Parallel input parsing, output writing and code-signature hashing,
  following the design described in the
  [mold paper](https://arxiv.org/abs/2608.23228)

## Architecture

The code mirrors mold's Rust port: `src/driver.rs` runs the passes in
order, `src/passes.rs` implements them, `src/arch/` isolates the
target-dependent relocation handling (arm64 and x86-64 are each
instantiated in a crate under `targets/` and dispatched by the
executable in `cli/`), and `src/output_chunks/` builds every piece of
the output file. Parsing is decoupled from resolution: all inputs,
including every archive member, are parsed in parallel, and symbol
resolution ranks competing definitions with a liveness walk deciding
which archive members join the link.

The commit history doubles as a reference on the Mach-O file format
and dyld: each commit's message explains the structures and loader
behavior involved in that change.

## Tests

Tests are shell scripts under `tests/cases/`, one feature per script,
driving the real toolchain through `cc --ld-path=...`; a harness runs
each for arm64 and (under Rosetta) x86-64:

    cargo test

## License

MIT, like mold.
