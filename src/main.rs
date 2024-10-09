mod analysis;

use analysis::Generator;
use clap::Parser;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
struct Args {
    /// The input JSON file (if not provided, will read from stdin)
    #[arg(short, long)]
    input: Option<String>,

    /// The output directory for generated files
    #[arg(short, long, default_value = "output")]
    output: String,

    /// Option to enable visualization (default: false)
    #[arg(short, long)]
    visualize: bool,

    /// The name of the parser (default: "schema_parser")
    #[arg(long, default_value = "schema_parser")]
    parser_name: String,

    /// The number of entries per chunk (epc) (default: 4)
    #[arg(long, default_value_t = 4)]
    epc: usize,

    /// The integer width (default: 64)
    #[arg(long, default_value_t = 64)]
    int_width: usize,
}

fn main() {
    let args = Args::parse();

    // Read the JSON input, either from a file or from stdin.
    let input_string = if let Some(input_path) = args.input {
        // Read from the provided input file
        fs::read_to_string(input_path).expect("Failed to read input file")
    } else {
        // Read from stdin if no input file is provided
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
        buffer
    };

    // Create a new generator
    let mut generator = Generator::new(&args.parser_name, args.epc, args.int_width);

    // Analyze the JSON string
    generator.analyze(&input_string).unwrap();

    if args.visualize {
        // Visualize the JSON string
        generator.visualize("output/schema.dot").unwrap();
    }

    // Generate TIL and TL code
    generator.generate(&args.output).unwrap();
}