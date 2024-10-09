# JSON-TIL

JSON-TIL is a tool for automating the design process for parsing JSON streams on FPGAs. It uses [Tydi-JSON](https://github.com/jhaenen/tydi-json) components, which implement JSON parsing components with Tydi streaming interfaces. JSON-TIL simplifies the process of assembling and configuring the parsing components required for specific JSON schema streams.

## Features

- **Automatic JSON Parsing Design**: JSON-TIL can analyze a sample JSON stream and automatically assemble the necessary parsing components.
- **Tydi-JSON Component Tree Visualizer**: Provides a visualization of the assembled Tydi-JSON component tree for better understanding and debugging.
- **TIL Generator**: Outputs a [Tydi Intermediate Language (TIL)](https://github.com/matthijsr/til-vhdl/) file, representing the parsing logic for the JSON stream.
- **TydiLang Generator**: In addition to TIL, JSON-TIL now supports generating [TydiLang](https://github.com/twoentartian/tydi-lang-2) (`.td`) files. TL can automatically insert stream duplicators, addressing the biggest issue in the original workflow.

## Workflow

1. **Input**: Provide a sample JSON stream to JSON-TIL.
2. **JSON Analysis**: The tool analyzes the JSON structure and identifies required parsing components.
3. **Component Assembly**: Automatically assembles and connects Tydi-JSON parsing components.
4. **Output**: Generates a TIL file and a TydiLang file that represent the parsing logic for the JSON stream.
5. **HDL Generation**:
   - Use the `til` file with [TIL-VHDL](https://github.com/matthijsr/til-vhdl/) to generate a VHDL project for FPGA implementation.
   - Use the `td` file with [TydiLang](https://github.com/twoentartian/tydi-lang-2) and [Tydi-lang-2-Chisel](https://github.com/ccromjongh/tydi-lang-2-chisel) to generate Chisel code and subsequently Verilog output for the FPGA implementation. See [Tydi-Chisel](https://github.com/abs-tudelft/Tydi-Chisel) for details.

## Usage

To use JSON-TIL, clone the repository and build the project:

```bash
git clone https://github.com/your-org/json-til.git
cd json-til
cargo build --release
```

## Requirements
- Rust (latest stable version)
- Python 3 (used by `pyo3`)
- TIL-VHDL or TydiLang for usage of the output

## Citing JSON-TIL
If you use JSON-TIL in your research, please cite it using the following BibTeX entry:
```tex
@article{haenen_json-til_2023,
    title = {{JSON}-{TIL}: {A} tool for generating/reducing boilerplate when creating and composing streaming {JSON} dataflow accelerators using {Tydi} interfaces},
    url = {https://github.com/jhaenen/JSON_hierachy/blob/master/TIL_JSON.pdf},
    author = {Haenen, Jasper},
    month = jan,
    year = {2023},
}
```

## License
This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for more details.
