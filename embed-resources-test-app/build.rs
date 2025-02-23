use std::path::Path;
use embed_resources::{Resource, ResourceContainer};

fn main() {
    let storage_dir = Path::new("embedded_data");
    let struct_output_path = Path::new("src/resource_container_embedded_data.rs"); // Struct file path
    let struct_name = "ResourceContainerEmbeddedData";

    let mut container = ResourceContainer::new(storage_dir, struct_output_path, struct_name);

    // Add resources from different sources
    container.add_resource("LOCAL_FILE", Resource::File("Cargo.toml".to_string()), true);
    container.add_resource(
        "REMOTE_FILE",
        Resource::Url("https://zenosmosis.com".to_string()),
        true,
    );
    container.add_resource(
        "ARBITRARY_DATA",
        Resource::Data(bytes::Bytes::from("Hello, world!")),
        true,
    );

    // Embed all resources into the "embed" directory with compression
    container.embed_all().expect("Could not embed resources");
}
