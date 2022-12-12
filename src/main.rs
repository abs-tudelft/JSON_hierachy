mod components;
mod visualization;
mod analysis;
mod vhdl;

use analysis::Generator;

fn main() {
    let multiple_keys = r#"
    {
        "voltage":
            [{"voltage":1128},{"voltage":1213},{"voltage":1850}],
        "valid":true,
        "current":
            {"current": 123}
     }
     "#;

    let nested = r#"
    {
        "voltage":
            [{"voltage":1128},{"voltage":1213},{"voltage":1850}]
    }
    "#;

    let simple = r#"
    {
        "voltage":
            [1128,1213,1850,429]
    }
    "#;

    let visualize = true;

    // Create a new generator
    let mut generator = Generator::new();

    // Analyze the JSON string
    generator.analyze(simple).unwrap();
    
    if visualize {
        // Visualize the JSON string
        generator.visualize("output/schema.dot").unwrap();
    }

    // Generate VHDL code
    generator.vhdl("output/schema.vhdl").unwrap();
}