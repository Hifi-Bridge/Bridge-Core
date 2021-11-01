use crate::Bytes;
use alloc::vec::Vec;
use core::ops::Deref;
use ethereum_types::{Address, H160, H256, U256};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use sha3::{Digest, Keccak256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransactionAction {
	Call(H160),
	Create,
}

impl Encodable for TransactionAction {
	fn rlp_append(&self, s: &mut RlpStream) {
		match self {
			Self::Call(address) => {
				s.encoder().encode_value(&address[..]);
			}
			Self::Create => s.encoder().encode_value(&[]),
		}
	}
}

impl Decodable for TransactionAction {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		if rlp.is_empty() {
			if rlp.is_data() {
				Ok(TransactionAction::Create)
			} else {
				Err(DecoderError::RlpExpectedToBeData)
			}
		} else {
			Ok(TransactionAction::Call(rlp.as_val()?))
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransactionRecoveryId(pub u64);

impl Deref for TransactionRecoveryId {
	type Target = u64;

	fn deref(&self) -> &u64 {
		&self.0
	}
}

impl TransactionRecoveryId {
	pub fn standard(self) -> u8 {
		if self.0 == 27 || self.0 == 28 || self.0 > 36 {
			((self.0 - 1) % 2) as u8
		} else {
			4
		}
	}

	pub fn chain_id(self) -> Option<u64> {
		if self.0 > 36 {
			Some((self.0 - 35) / 2)
		} else {
			None
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "with-codec", derive(scale_info::TypeInfo))]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransactionSignature {
	v: TransactionRecoveryId,
	r: H256,
	s: H256,
}
