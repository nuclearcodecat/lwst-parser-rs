pub struct Lwst {
	pub version: u8,
	pub encoding: EncodingType,
	pub compression: Option<CompressionKind>,
	pub software_color: bool,
	pub software_pos: bool,
	pub software_font: bool,
}

pub enum LwstError {
	UnexpectedEnd,
	NotLwst,
	GenericParseError,
}

pub enum EncodingType {
	Utf8,
	Ascii,
}

#[allow(dead_code)]
pub enum CompressionKind {
}

impl Lwst {
	pub fn new(s: &[u8]) -> Result<Lwst, LwstError> {
		// skip 4 for the magic check
		let mut iter = s.iter().skip(4);

		if !s.starts_with(b"lwst") {
			return Err(LwstError::NotLwst);
		}

		let version = *iter.next().ok_or(LwstError::UnexpectedEnd)?;
		
		let encoding = match iter.next() {
			Some(0) => EncodingType::Utf8,
			Some(1) => EncodingType::Ascii,
			_ => return Err(LwstError::GenericParseError),
		};

		// todo
		let compression: Option<CompressionKind> = match iter.next() {
			Some(_) => None,
			_ => return Err(LwstError::GenericParseError),
		};

		let software_color = *iter.next().ok_or(LwstError::UnexpectedEnd)? != 0;
		let software_pos = *iter.next().ok_or(LwstError::UnexpectedEnd)? != 0;
		let software_font = *iter.next().ok_or(LwstError::UnexpectedEnd)? != 0;

		Ok(Lwst {
			version,
			encoding,
			compression,
			software_color,
			software_pos,
			software_font,
		})
	}
}
