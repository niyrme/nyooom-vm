pub trait ToBytes {
	fn bytes(&self) -> Vec<u8>;
}

pub(crate) trait FromBytes {
	fn fromBytes(bytes: &mut Vec<u8>) -> Self;
}
