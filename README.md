# An Interpreted Data Processing DSL in Rust

a domain-specific language (DSL) for building **data processing pipelines**.  
It lets you describe operations like filtering, grouping, aggregation, and output in a clean, human-readable `.dpl`??`.flow` file.  
The interpreter, written in Rust, parses the DSL into an AST and executes it directly using [Polars](https://www.pola.rs/).

## Features

- **Custom DSL File Type**: `.dpl` for declarative data pipelines.
- **Interpreter in Rust**: Parses DSL → AST → executes directly.
- **Data Sources**: CSV and JSON.
- **Operations**:
  - `source` – load data
  - `filter` – apply row conditions
  - `group by` – group data
  - `aggregate` – compute metrics
  - `output` – write results
- **Command-Line Tool**:
  - `dpl main.dpl` – run a pipeline
  - `dpl repl` – interactive mode
  - `dpl explain main.dpl` – print parsed AST

## Example DSL Pipeline

`sales.dpl`:

```dpl
csv <- "sales.csv"
filter(where amount > 1000)
group by region
aggregate(sum(amount) as total_sales)
output -> "report.json"
```

This pipeline:

1. Loads `sales.csv`
2. Filters rows where `amount > 1000`
3. Groups data by `region`
4. Aggregates `amount` into `total_sales`
5. Writes the result to `report.json`

## Output Example

`report.json`:

```json
[
  { "region": "North", "total_sales": 124000 },
  { "region": "South", "total_sales": 89000 }
]
```

## Installation

Build the binary with Cargo:

```bash
cargo build --release
```

Move the binary to your PATH:

```bash
cp target/release/dpl ~/.cargo/bin/
```

Now you can run DPL from anywhere:

```bash
dpl main.dpl
```

## Usage

Run a `.dpl` pipeline:

```bash
dpl main.dpl
```

Start an interactive REPL:

```bash
dpl repl
```

Debug/inspect the parsed AST:

```bash
dpl explain main.dpl
```

## Project Structure

```
dpl/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point
│   ├── parser/          # DSL parser (Pest/Nom grammar)
│   ├── ast.rs           # AST definitions
│   ├── interpreter.rs   # Interpreter engine
│   ├── errors.rs        # Custom error types
│   └── utils.rs         # Helper functions
├── examples/
│   └── sales_pipeline.dpl
└── README.md
```

## Future Enhancements

- Expressions (`amount * 1.1`, derived columns).
- More file formats (Parquet, Excel).
- Built-in functions (mean, median, variance).
- Better error reporting with line/column context.
- Optimizations: merge filters, lazy evaluation.

## License

MIT License © 2025 sparrowsaurora

## Contact

\> [github](https://github.com/sparrowsaurora)  
\> [email](mailto:sparrows.au@gmail.com)

([Back to Top](#))
`
