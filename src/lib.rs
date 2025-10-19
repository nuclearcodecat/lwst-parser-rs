use thiserror::Error;

#[derive(Debug)]
pub struct Lwst {
	pub version: u8,
	pub encoding: EncodingType,
	pub compression: Option<CompressionKind>,
	pub simple_font_table: bool,
	pub software_color: bool,
	pub software_pos: bool,
	pub software_font: bool,
	pub color_table: Option<Vec<ColorTableEntry>>,
	pub pos_table: Option<Vec<PosTableEntry>>,
	pub font_table: Option<FontTable>,
	pub subtitle_table: Vec<String>,
}

#[derive(Error, Debug)]
pub enum LwstError {
	#[error("unexpected end of lwst stream")]
	UnexpectedEnd,
	#[error("not valid lwst")]
	NotLwst,
	#[error("illegal value in field")]
	IllegalValue,
	#[error("utf-8 error: {0}")]
	Utf8(#[from] std::string::FromUtf8Error),
}

#[derive(Debug)]
pub enum EncodingType {
	Utf8,
	Ascii,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CompressionKind {
}

#[derive(Debug)]
pub struct ColorTableEntry {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[repr(u8)]
#[derive(Debug)]
pub enum AnchorPoint {
	TopLeft,
	TopCenter,
	TopRight,
	MiddleLeft,
	Center,
	MiddleRight,
	BottomLeft,
	BottomCenter,
	BottomRight,
}

#[derive(Debug)]
pub struct ComplexFontTableEntry {
	pub font: String,
	pub weight: u16,
}

#[derive(Debug)]
pub enum FontTable {
	Simple(Vec<u8>),
	Complex(Vec<ComplexFontTableEntry>),
}

#[derive(Debug)]
pub struct PosTableEntry {
	pub anchor_point: AnchorPoint,
	pub x_attachment_point: u8,
	pub y_attachment_point: u8,
	pub max_box_width: Option<u8>,
}

macro_rules! next {
	($iter: expr) => {
		$iter.next().ok_or(LwstError::UnexpectedEnd)?
	};
}

impl Lwst {
	pub fn new(s: &[u8]) -> Result<Lwst, LwstError> {
		// skip 4 for the magic check
		let mut iter = s.iter().skip(4).peekable();

		// do magic check
		if !s.starts_with(b"lwst") {
			return Err(LwstError::NotLwst);
		}

		let version = *next!(iter);
		
		let encoding = match iter.next() {
			Some(0) => EncodingType::Utf8,
			Some(1) => EncodingType::Ascii,
			_ => return Err(LwstError::UnexpectedEnd),
		};

		// todo
		let compression: Option<CompressionKind> = match iter.next() {
			Some(_) => None,
			_ => return Err(LwstError::UnexpectedEnd),
		};

		let simple_font_table = *next!(iter) != 0;
		let software_color = *next!(iter) != 0;
		let software_pos = *next!(iter) != 0;
		let software_font = *next!(iter) != 0;

		let color_table_len = *next!(iter);
		let color_table = if color_table_len > 0 {
			let mut color_table: Vec<ColorTableEntry> = vec![];
			for _ix in 0..color_table_len {
				let r = *next!(iter);
				let g = *next!(iter);
				let b = *next!(iter);
				let entry = ColorTableEntry { r, g, b };
				color_table.push(entry);
			}
			Some(color_table)
		} else {
			None
		};
		
		let pos_table_len = *next!(iter);
		let pos_table = if pos_table_len > 0 {
			let mut pos_table: Vec<PosTableEntry> = vec![];
			for _ix in 0..pos_table_len {
				let anchor_point = *next!(iter);
				println!("{}", anchor_point);
				let anchor_point = if anchor_point > 8 {
					return Err(LwstError::IllegalValue);
				} else {
					// i'm not writing a huge ugly match statement
					unsafe { std::mem::transmute::<u8, AnchorPoint>(anchor_point) }
				};
				let x_attachment_point = *next!(iter);
				let y_attachment_point = *next!(iter);
				let max_box_width = iter.next().copied();
				let entry = PosTableEntry { anchor_point, x_attachment_point, y_attachment_point, max_box_width };
				println!("{:#?}", entry);
				pos_table.push(entry);
			};

			Some(pos_table)
		} else {
			None
		};

		let font_table_len = *next!(iter);
		let font_table = if font_table_len > 0 {
			let font_table = if simple_font_table {
				let mut vec = vec![];
				for _ix in 0..font_table_len {
					vec.push(*next!(iter));
				}
				FontTable::Simple(vec)
			} else {
				let mut vec = vec![];
				for _ix in 0..font_table_len {
					let font = String::from_utf8(iter.by_ref().take_while(|&&b| b != 0x00).copied().collect())?;
					let weight = *next!(iter) as u16 * 100;
					let entry = ComplexFontTableEntry {
						font, weight
					};
					vec.push(entry);
				}
				FontTable::Complex(vec)
			};
			Some(font_table)
		} else {
			None
		};

		let subtitle_table = {
			let mut vec = vec![];
			while iter.peek().map(|&&b| b != 0x00).ok_or(LwstError::UnexpectedEnd)? {
				let sub = String::from_utf8(iter.by_ref().take_while(|&&b| b != 0x00).copied().collect())?;
				vec.push(sub);
			}
			vec
		};

		Ok(Lwst {
			version,
			encoding,
			compression,
			simple_font_table,
			software_color,
			software_pos,
			software_font,
			color_table,
			pos_table,
			font_table,
			subtitle_table,
		})
	}
}
