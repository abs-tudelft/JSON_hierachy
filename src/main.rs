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

    let visualize = true;

    // Create a new generator
    let mut generator = Generator::new();

    // Analyze the JSON string
    generator.analyze(_multiple_keys).unwrap();
    
    if visualize {
        // Visualize the JSON string
        generator.visualize("output/schema.dot").unwrap();
    }

    // Generate VHDL code
    // generator.vhdl("output/schema.vhdl").unwrap();

    // Generate TIL code
    generator.til("output/schema.til").unwrap();
}