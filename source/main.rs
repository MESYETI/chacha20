#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_upper_case_globals)]

mod chacha;
mod keyStream;

use hex;

use crate::keyStream::KeyStream;

fn main() {
	let mut rand = urandom::rng::System::<31>::new();

	let mut key:   [u8; 32] = [0; 32];
	let mut nonce: [u8; 8]  = [0; 8];

	rand.fill_bytes(&mut key);
	rand.fill_bytes(&mut nonce);

	let mut keyStream = KeyStream::new(&key, &nonce, 20);

	let message   = "Hello world!";
	let encrypted = keyStream.run(message.as_bytes());

	keyStream.reset();
	let decrypted = keyStream.run(&encrypted);

	println!("message:   {}", message);
	println!("encrypted: {}", hex::encode(&encrypted));
	println!("decrypted: {}", str::from_utf8(&decrypted).unwrap());
}
