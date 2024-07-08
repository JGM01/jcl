# JCL: Jacob's C Lexer
JCL (Jacob's C Lexer) is a robust and efficient lexical analyzer for the C programming language, implemented in Rust. It's designed to tokenize C source code into a stream of meaningful tokens, serving as the foundation for further parsing and compilation processes.

### Key Features
- **Comprehensive Token Support**: Recognizes and categorizes a wide range of C language elements including keywords, identifiers, literals, operators, and punctuators.
- **Comment Handling**: Properly handles both single-line (//) and multi-line (/* */) comments.
- **Flexible Output**: Offers various output options including token positions, token counts, and verbose mode for detailed analysis.
- **Error Reporting**: Provides clear error messages with line and column information for invalid tokens or unexpected characters.
- **Performance Metrics**: Includes optional diagnostics for runtime and memory usage.

### Installation
To use JCL, you'll need Rust installed on your system. If you don't have Rust, you can install it from [rust-lang.org](https://www.rust-lang.org).
Clone the repository and build the project:
```bash
git clone https://github.com/JGM01/jcl.git
cd jcl
cargo build --release
```

### Usage
Run JCL on a C source file:
```bash
cargo run -- path/to/file.c
```

#### Command-Line Options
- `--no-comments`: Exclude comments from the output
- `-p`, `--show-positions`: Show token positions in the output
- `-c`, `--count-tokens`: Display a count of each token type
- `-v`, `--verbose`: Use verbose output
- `-d`, `--diagnostics`: Show system diagnostics (runtime and memory usage)

Example with options:
```bash
cargo run -- path/to/file.c --no-comments -p -c -v -d
```

### Running Tests
To run tests:
```bash
cargo test
```
