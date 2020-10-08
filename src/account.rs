use ethereum_types::{H256, U256};
use rlp_derive::{RlpDecodable, RlpEncodable};

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
#[cfg_attr(feature = "codec", derive(codec::Encode, codec::Decode))]
pub struct Account {
	pub nonce: U256,
	pub balance: U256,
	pub storage_root: H256,
	pub code_hash: H256,
}