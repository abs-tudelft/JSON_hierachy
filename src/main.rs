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

     let _student = r#"
     {
        "student_number": "S123456789",
        "name": "John Doe",
        "birthdate": "2000-05-15",
        "study_start": "2021-05-15",
        "study_end": null,
        "study": "Computer Science",
        "email": "john.doe@example.com",
        "exams": [
          {
            "course_code": "CS101",
            "course_name": "Introduction to Computer Science",
            "exam_date": "2023-12-10",
            "grade": 80
          },
          {
            "course_code": "MATH201",
            "course_name": "Calculus",
            "exam_date": "2023-12-15",
            "grade": 60
          },
          {
            "course_code": "ENG101",
            "course_name": "English Composition",
            "exam_date": "2023-12-20",
            "grade": 73
          }
        ]
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
    let mut generator = Generator::new("student_schema_parser", 4, 64);

    // Analyze the JSON string
    generator.analyze(_student).unwrap();
    
    if visualize {
        // Visualize the JSON string
        generator.visualize("output/schema.dot").unwrap();
    }

    // Generate TIL code
    generator.generate("output").unwrap();
}