mod embedded_data;
use embedded_data::EmbeddedData;

fn main() {
    println!("Hello, world! {:?} {:?}", EmbeddedData::ARRAY_ONE, EmbeddedData::ARRAY_TWO);
}
