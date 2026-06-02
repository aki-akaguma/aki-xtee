# Code Review Report: aki-xtee

## 1. Introduction
This report provides a technical code review of the `aki-xtee` project, a Rust-based `tee` clone with integrated compression support. The review focuses on architectural choices, idiomatic Rust usage, and functional parity with the original `tee` command.

## 2. Architecture and Design

### 2.1 Line-Oriented I/O (Main Concern)
The current implementation in `src/run.rs` processes input line-by-line using `sioe.pg_in().lines()`.
- **Incompatibility with Binary Data:** Standard `tee` is a byte-stream tool. `aki-xtee` is restricted to UTF-8 text. Attempting to process binary data containing invalid UTF-8 sequences will result in an error.
- **Line Ending Normalization:** By using `lines()`, the tool strips original line endings and forces a `\n` (or whatever `Runnel` uses) upon output. This can corrupt files that rely on specific line endings (e.g., CRLF).
- **Performance:** Reading and writing `String` objects for every line is significantly more expensive than processing raw byte chunks (`[u8]`).

**Recommendation:** Refactor the core loop to use byte-chunk processing (e.g., `std::io::copy` or a custom buffer loop) to ensure compatibility with all data types and improve performance.

### 2.2 I/O Abstraction
The use of `runnel` for I/O abstraction is consistent throughout the project. It provides a clean way to handle standard streams and facilitates testing.

## 3. Implementation Details

### 3.1 Append Mode (-a) Issues
- **Missing File Failure:** Standard `tee -a` creates the file if it doesn't exist. Currently, `aki-xtee` fails with "No such file or directory" in this scenario because `File::options().append(true).open(path)` is used without `.create(true)`.
- **Documentation Sync:** `src/lib.rs` documents the `-a` option as `[unimplemented]`, while the actual implementation in `src/util/adapt_output.rs` and the generated help message suggest it is available.

**Recommendation:** Add `.create(true)` to the file options in `make_output` and synchronize the documentation in `src/lib.rs`.

### 3.2 Compression Wrapper Ownership
The compression wrappers (e.g., `GzEnc`, `XzEnc`) use a `take()`/`replace()` pattern with `Option` on every `write` and `flush` call.
- This pattern is slightly unusual and incurs a small overhead.
- In `PlainOut::write`, returning `Ok(0)` when the inner writer is missing is technically incorrect for a `Write` implementation, which should usually return an error if it cannot perform the write.

**Recommendation:** Consider using `self.0.as_mut().expect(...)` if the `Option` is guaranteed to be `Some` until `finish()` is called. Ensure that `write` and `flush` return an error if called after the writer has been consumed by `finish()`.

## 4. Documentation and User Experience

### 4.1 Documentation Consistency
There are several mismatches between the source code doc comments, the `xtask` command definitions, and the generated help text. Specifically, `-a` and `-p` options have conflicting "unimplemented" statuses.

### 4.2 Feature Parity
While `aki-xtee` adds valuable compression features, its current text-only nature makes it a "subset" rather than a "superset" of `tee`.

## 5. Conclusion
`aki-xtee` is a well-structured project that successfully integrates multiple compression formats into a `tee`-like utility. However, the shift to line-oriented processing limits its general utility. Addressing the binary data support and fixing the append-mode bug would significantly enhance its value as a drop-in replacement for `tee`.

---
Review Date: 2026-06-02
Reviewer: Gemini CLI Agent
