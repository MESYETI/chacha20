use crate::chacha::ChaCha;

pub struct KeyStream {
	chacha:  ChaCha,
	stream:  Vec<u8>,
	counter: u64
}

impl KeyStream {
	pub fn new(key: &[u8; 32], nonce: &[u8; 8], rounds: usize) -> KeyStream {
		return KeyStream {
			chacha:  ChaCha::new(key, nonce, rounds),
			stream:  Vec::new(),
			counter: 0
		};
	}

	pub fn next_byte(&mut self) -> u8 {
		if self.stream.is_empty() {
			self.chacha.set_counter(self.counter);

			self.stream   = self.chacha.block().to_vec();
			self.counter += 1;
		}

		return self.stream.remove(0);
	}

	pub fn run(&mut self, data: &[u8]) -> Vec<u8> {
		let mut ret = Vec::new();

		for byte in data {
			ret.push(byte ^ self.next_byte());
		}

		return ret;
	}

	pub fn reset(&mut self) {
		self.counter = 0;
		self.stream  = Vec::new();
	}
}
