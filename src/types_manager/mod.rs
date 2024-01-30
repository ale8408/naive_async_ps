#[derive(Debug)]
pub struct PortRange {
	pub start: u16,
	pub end: u16,
}

impl PortRange {
	pub fn new(
		start: u16,
		end: u16,
	) -> Self {
		Self {
			start, end
		}
	}
}