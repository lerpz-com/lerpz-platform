use std::{fs::File, io::Read, path::Path, sync::LazyLock};

use jsonwebtoken::{DecodingKey, EncodingKey};

pub static JWT_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
	let path = Path::new("./keys/ed25519_public.pem");
	let file = File::open(path).unwrap();

	let mut reader = std::io::BufReader::new(&file);
	let mut bytes = Vec::new();

	reader.read_to_end(&mut bytes).unwrap();
	DecodingKey::from_ed_pem(&bytes).unwrap()
});

pub static JWT_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
	let path = Path::new("./keys/ed25519_private.pem");
	let file = File::open(path).unwrap();

	let mut reader = std::io::BufReader::new(&file);
	let mut bytes = Vec::new();

	reader.read_to_end(&mut bytes).unwrap();
	EncodingKey::from_ed_pem(&bytes).unwrap()
});
