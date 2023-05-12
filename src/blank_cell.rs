
// Blank Cell used to prioritize solving tree.
#[derive(Debug, PartialEq)]
pub struct BlankCell {
	pub index: usize,
	pub possible_values: Vec<u8>
}