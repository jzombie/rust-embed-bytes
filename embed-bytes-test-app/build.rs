use std::path::Path;
use bytes::Bytes;
use embed_bytes::write_byte_arrays;

fn main() {
    let struct_output_path = Path::new("src/embedded_data.rs"); // Struct file path
    let struct_name = "EmbeddedData";
    let byte_arrays = vec![
        ("ARRAY_ONE", Bytes::from(vec![1, 2, 3, 4])),
        ("ARRAY_TWO", Bytes::from(vec![5, 6, 7, 8])),
    ];

    match write_byte_arrays(struct_output_path, struct_name, byte_arrays) {
        Ok(()) => println!("Byte arrays and struct written successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
