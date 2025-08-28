use base58::ToBase58;
use blake3;
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub fn generate_wallet() -> (SigningKey, String) {
  let signing_key = SigningKey::random(&mut OsRng);
  let verifying_key = VerifyingKey::from(&signing_key);

  let point = verifying_key.to_encoded_point(true);
  let pubkey_bytes = point.as_bytes();

  let addr_bytes = [&[0x00], &blake3::hash(pubkey_bytes).as_bytes()[..24]].concat();
  let hash_result = blake3::hash(&addr_bytes);
  let checksum = &hash_result.as_bytes()[..4];
  let address = [&addr_bytes, checksum].concat().to_base58();

  (signing_key, address)
}