
// Blank Cell used to prioritize solving tree.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct BlankCell {
	pub index: u8,
	pub possible_values: Vec<u8>
}