mod resource_container_embedded_data;
use resource_container_embedded_data::ResourceContainerEmbeddedData;

fn format_bytes(data: &[u8]) -> String {
    let preview: Vec<String> = data.iter().take(5).map(|b| format!("{:02X}", b)).collect();
    if data.len() > 5 {
        format!("[{}...]", preview.join(", "))
    } else {
        format!("[{}]", preview.join(", "))
    }
}

fn main() {
    println!(
        "LOCAL_FILE: {:?}",
        format_bytes(ResourceContainerEmbeddedData::LOCAL_FILE)
    );
    println!(
        "REMOTE_FILE: {:?}",
        format_bytes(ResourceContainerEmbeddedData::REMOTE_FILE)
    );
    println!(
        "ARBITRARY_DATA: {:?}",
        format_bytes(ResourceContainerEmbeddedData::ARBITRARY_DATA)
    );
}
