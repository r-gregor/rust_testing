// use base64::Engine;
use std::str;
use std::error::Error;
use base64::{Engine as _, engine::{general_purpose}};

extern crate base64;

fn main() -> Result<(), Box<dyn Error>>{

	let b64 = "V2VsY29tZSB0byBMaW51eGhpbnQ=";
	// let decoded = &base64::Engine::decode(b64).unwrap()[..];
	let decoded = &general_purpose::STANDARD.decode(b64).unwrap()[..];
	println!("String: {:?}", str::from_utf8(decoded));

	Ok(())
}
