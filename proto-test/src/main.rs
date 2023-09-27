use std::fs::read;
use pb::{Empty, TestMessage};
use prost::Message;

#[path = "../src/test.rs"]
mod pb;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let buf = read("../proto-bin")?;

    let test_message = TestMessage::decode(buf.as_slice())?;

    dbg!(test_message);

    Ok(())
}
