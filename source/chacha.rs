pub struct ChaCha {
	pub state:   [u32; 16],
	pub rounds:  usize
}

fn encode_const(value: &str) -> u32 {
	let bytes: [u8; 4] = value.as_bytes().try_into().expect("invalid");
	return u32::from_le_bytes(bytes);
}

fn rotate(a: u32, b: u32) -> u32 {
	return a << b | (a >> (32 - b));
}

impl ChaCha {
	pub fn new(keyBytes: &[u8; 32], nonceBytes: &[u8; 8], rounds: usize) -> ChaCha {
		let key: [u32; 8] = [
			u32::from_le_bytes(keyBytes[0  .. 4].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[4  .. 8].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[8  .. 12].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[12 .. 16].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[16 .. 20].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[20 .. 24].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[24 .. 28].try_into().unwrap()),
			u32::from_le_bytes(keyBytes[28 .. 32].try_into().unwrap())
		];
		let nonce: [u32; 2] = [
			u32::from_le_bytes(nonceBytes[0 .. 4].try_into().unwrap()),
			u32::from_le_bytes(nonceBytes[4 .. 8].try_into().unwrap())
		];

		return ChaCha {
			state: [
				encode_const("expa"), encode_const("nd 3"),
				encode_const("2-by"), encode_const("te k"),
				key[0], key[1], key[2], key[3],
				key[4], key[5], key[6], key[7],
				0, 0, nonce[0], nonce[1]
			],
			rounds: rounds
		};
	}

	pub fn set_counter(&mut self, counter: u64) {
		let counterBytes: [u32; 2] = [
			counter as u32, (counter >> 32) as u32
		];

		self.state[12] = counterBytes[0];
		self.state[13] = counterBytes[1];
	}

	pub fn block(&mut self) -> [u8; 64] {
		let mut block: [u32; 16] = self.state;

		let mut qr = |a: usize, b: usize, c: usize, d: usize| {
			block[a]  = block[a].wrapping_add(block[b]);
			block[d] ^= block[a];
			block[d]  = rotate(block[d], 16);

			block[c]  = block[c].wrapping_add(block[d]);
			block[b] ^= block[c];
			block[b]  = rotate(block[b], 12);

			block[a]  = block[a].wrapping_add(block[b]);
			block[d] ^= block[a];
			block[d]  = rotate(block[d], 8);

			block[c]  = block[c].wrapping_add(block[d]);
			block[b] ^= block[c];
			block[b]  = rotate(block[b], 7);
		};

		for _ in 0 .. self.rounds / 2 {
			// odd round
			qr(0, 4,  8, 12);
			qr(1, 5,  9, 13);
			qr(2, 6, 10, 14);
			qr(3, 7, 11, 15);
			// even round
			qr(0, 5, 10, 15);
			qr(1, 6, 11, 12);
			qr(2, 7,  8, 13);
			qr(3, 4,  9, 14);
		}

		let mut ret = Vec::new();

		for word in block {
			ret.extend_from_slice(&word.to_le_bytes());
		}

		return ret.try_into().unwrap();
	}
}
