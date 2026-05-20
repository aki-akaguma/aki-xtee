# Code Review for `aki-xtee`

## 1. Introduction
The `aki-xtee` project is a Rust-based CLI utility that extends the functionality of the Unix `tee` command by adding automatic compression support based on file extensions. This review evaluates the codebase's architecture, efficiency, correctness, and idiomatic use of Rust.

## 2. Architecture and Design
The project demonstrates a solid modular architecture:
- **Separation of Concerns**: Logic is well-partitioned into argument parsing (`conf`), core processing (`run`), and I/O utilities (`util`).
- **Extensibility**: The use of the Strategy pattern via the `Finish` trait and `Box<dyn Finish>` allows for easy addition of new compression formats.
- **Factory Pattern**: `util::adapt_output::open_files` effectively decouples the main loop from the details of file creation and compression initialization.

## 3. Critical Issues

### 3.1. Inefficient Line-Based I/O
In `src/run.rs`, the `run_0` function processes input using `sioe.pg_in().lines()`:
```rust
for line in sioe.pg_in().lines() {
    let line_s = line?;
    let line_ss = line_s.as_str();
    for file in file_vec.iter_mut() {
        file.write_fmt(format_args!("{line_ss}\n"))?;
    }
    sioe.pg_out().write_line(line_s)?;
}
```
**Issues:**
1. **Performance**: Processing a file line-by-line is significantly slower than chunk-based I/O for large datasets.
2. **Binary Data Corruption**: `lines()` assumes UTF-8 encoded text and strips line endings. Using `format_args!("{line_ss}\n")` to add them back will corrupt binary data and may alter the original line endings (e.g., converting CRLF to LF).
3. **Recommendation**: Implement a byte-oriented loop using a buffer (e.g., `std::io::copy` or a manual loop with `read()` and `write_all()`) to ensure the tool behaves like a true `tee` replacement.

### 3.2. Performance Overhead in Compression Encoders
The compression wrappers (e.g., `GzEnc` in `src/util/compress/gz.rs`) use an `Option::take()` and `Option::replace()` pattern for every `write` and `flush` call:
```rust
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    if let Some(mut a) = self.0.take() {
        let r = a.write(buf);
        let _ = self.0.replace(a);
        r
    } else {
        Ok(0)
    }
}
```
**Issues:**
- This adds unnecessary branching and state manipulation for every single write operation.
**Recommendation**: Use `self.0.as_mut().expect("...")` or `self.0.as_mut().unwrap()` for `write` and `flush`. The `Option` is only necessary to consume the encoder in the `finish()` method.

## 4. Correctness and Minor Improvements

### 4.1. Typo in Error Message
In `src/util/adapt_output.rs`, there is a typo in the error message for `.xz` files:
```rust
bail!("not support '.xy' by compile option");
```
It should be `.xz`.

### 4.2. Unused Field in `NameWrite`
The `name` field in the `NameWrite` struct (in `src/util/adapt_output.rs`) is marked with `#[allow(dead_code)]` and is never used.
**Recommendation**: Either use this field for better error reporting (e.g., "failed to write to <name>") or remove it to simplify the struct.

### 4.3. Redundant `BufWriter`
The `PlainOut` encoder uses `BufWriter<File>`. If the main loop is changed to use a large buffer (e.g., 64KB), this additional layer of buffering might be redundant and slightly decrease performance.

## 5. CLI and Documentation
- **Unimplemented Features**: The CLI help lists `-a, --append` and `-p, --pipe-out` as `[unimplemented]`. While honest, it's generally better to either implement these or hide them from the help until they are ready, to avoid user confusion.
- **Library Documentation**: The crate-level documentation in `src/lib.rs` is excellent, providing clear examples and installation instructions.

## 6. Conclusion
`aki-xtee` is a well-structured tool with a clear purpose. However, the transition from line-based processing to byte-oriented chunked processing is essential for it to be a reliable and high-performance utility capable of handling arbitrary data. Refining the compression wrapper implementations will also provide a modest performance boost.

---
Review Date: 2026-05-20
Reviewer: Gemini CLI Agent
