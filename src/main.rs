mod analysis;

use analysis::Generator;

fn main() {
    let _multiple_keys = r#"
    {
        "temperature":
            [{"voltage":1128},{"voltage":1213},{"voltage":1850}],
        "valid":true,
        "humidity":
            [{"voltage":567},{"voltage":234},{"voltage":1230}]
     }
     "#;

    let _nested = r#"
    {
        "voltage":
            [{"voltage":1128},{"voltage":1213},{"voltage":1850}]
    }
    "#;

    let _simple = r#"
    {
        "voltage":
            [1128,1213,1850,429]
    }
    "#;

    let visualize = false;

    // Create a new generator
    let mut generator = Generator::new("schema_parser", 4, 8, 64);

    // Analyze the JSON string
    generator.analyze(_simple).unwrap();
    
    if visualize {
        // Visualize the JSON string
        generator.visualize("output/schema.dot").unwrap();
    }

    // Generate VHDL code
    // generator.vhdl("output/schema.vhdl").unwrap();

    // Generate TIL code
    generator.generate("output/schema.til").unwrap();
}