use std::{
	mem,
	fmt,
	io::{Error, ErrorKind}
};
pub use crate::point::Point;
pub use crate::image::SimpleImage;

const FILE_HEADER_SIZE: usize = 14;
const INFO_HEADER_SIZE: usize = 40;

#[derive(Clone)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}

#[derive(Debug)]
pub struct FileHeader {
	pub file_type: [u8; 2],
	pub file_size: u32,
	pub reserved: u32,
	pub data_offset: u32
}

#[derive(Debug)]
pub struct InfoHeader {
	pub info_header_size: u32,
	pub width: i32,
	pub height: i32,
	pub planes: u16,
	pub bits_per_pixel: u16,
	pub compression: u32,
	pub image_size: u32,
	pub x_pixels_per_meter: i32,
	pub y_pixels_per_meter: i32,
	pub used_colors: u32,
	pub important_colors: u32
}

pub struct Image {
	pub file_header: FileHeader,
	pub info_header: InfoHeader,
	pub pixels: Vec<u32>
}

impl Color {
	fn new() -> Color {
		Color {
			alpha: 0,
			red: 0,
			green: 0,
			blue: 0
		}
	}
	pub fn from(color: u32) -> Color {
		let color = color.to_be_bytes();
		Color {
			alpha: color[0],
			red: color[1],
			green: color[2],
			blue: color[3]
		}
	}
	pub fn to(&self) -> u32 {
		u32::from_be_bytes([
			self.alpha,
			self.red,
			self.green,
			self.blue
		])
	}
}

impl FileHeader {
	pub fn new() -> FileHeader {
		FileHeader {
			file_type: [0; 2],
			file_size: 0,
			reserved: 0,
			data_offset:
				(FILE_HEADER_SIZE +
				INFO_HEADER_SIZE)
				.try_into().unwrap()
		}
	}
	pub fn from(bytes: &[u8; FILE_HEADER_SIZE]) -> FileHeader {
		let mut object = FileHeader::new();
		let mut left;
		let mut right = 0;
		
		left = right;
		right += mem::size_of_val(&object.file_type);
		object.file_type =
			bytes[left..right].try_into().unwrap();
		
		left = right;
		right += mem::size_of_val(&object.file_size);
		object.file_size =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());
		
		left = right;
		right += mem::size_of_val(&object.reserved);
		object.reserved =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());
		
		left = right;
		right += mem::size_of_val(&object.data_offset);
		object.data_offset =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());

		object
	}
}

impl InfoHeader {
	pub fn new() -> InfoHeader {
		InfoHeader {
			info_header_size: INFO_HEADER_SIZE.try_into().unwrap(),
			width: 0,
			height: 0,
			planes: 1,
			bits_per_pixel: 32,
			compression: 0,
			image_size: 0,
			x_pixels_per_meter: 0,
			y_pixels_per_meter: 0,
			used_colors: 0,
			important_colors: 0
		}
	}
	pub fn from(bytes: &[u8; INFO_HEADER_SIZE]) -> InfoHeader {
		let mut object = InfoHeader::new();
		let mut left;
		let mut right = 0;

		left = right;
		right += mem::size_of_val(&object.info_header_size);
		object.info_header_size =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.width);
		object.width =
			i32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.height);
		object.height =
			i32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.planes);
		object.planes =
			u16::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.bits_per_pixel);
		object.bits_per_pixel =
			u16::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.compression);
		object.compression =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.image_size);
		object.image_size =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.x_pixels_per_meter);
		object.x_pixels_per_meter =
			i32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.y_pixels_per_meter);
		object.y_pixels_per_meter =
			i32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.used_colors);
		object.used_colors =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());

		left = right;
		right += mem::size_of_val(&object.important_colors);
		object.important_colors =
			u32::from_le_bytes(bytes[left..right].try_into().unwrap());

		object
	}
}

impl Image {
	pub fn new() -> Image {
		Image {
			file_header: FileHeader::new(),
			info_header: InfoHeader::new(),
			pixels: Vec::<u32>::new()
		}
	}
	pub fn open(file_path: &str) -> Result<Image, Error> {
		let bytes = std::fs::read(file_path)?;
		if bytes.len() < FILE_HEADER_SIZE + INFO_HEADER_SIZE {
			return Err(Error::new(
				ErrorKind::InvalidData,
				format!("Image files must be at least 54 bytes, but this one is {}", bytes.len())
			));
		}
		let mut object = Image::new();
		let mut left;
		let mut right = 0;
		
		left = right;
		right += FILE_HEADER_SIZE;
		object.file_header =
			FileHeader::from(bytes[left..right].try_into().unwrap());

		left = right;
		right += INFO_HEADER_SIZE;
		object.info_header =
			InfoHeader::from(bytes[left..right].try_into().unwrap());
		
		if object.file_header.file_size != bytes.len().try_into().unwrap() {
			return Err(Error::new(
				ErrorKind::InvalidData,
				format!(
					"file metadata suggests its size is {reported_size} bytes, but it is actually {actual_size} bytes",
					reported_size=object.file_header.file_size,
					actual_size=bytes.len()
				)
			));
		}
		
		if object.info_header.bits_per_pixel != 24 && object.info_header.bits_per_pixel != 32 {
			return Err(Error::new(
				ErrorKind::InvalidData, 
				format!(
					"only images with 32 and 24 bits per pixel supported, this one has {}",
					object.info_header.bits_per_pixel)
			));
		}

		let width: usize = object.info_header.width.try_into().expect("bmp image width is negative");
		let height: usize = object.info_header.height.try_into().expect("bmp image height is negative");
		let data_offset: usize = object.file_header.data_offset.try_into().unwrap();
		let bytes_per_pixel: usize = (object.info_header.bits_per_pixel / 8).try_into().unwrap();
		let row_size = width * bytes_per_pixel;

		if data_offset + (row_size + row_size % 4) * height != bytes.len() {
			return Err(Error::new(
				ErrorKind::InvalidData,
				format!(
					"image's \
					width({width})\
					, height({height})\
					, bits per pixel({bits_per_pixel})\
					, padding({padding} bytes) \
					and data offset({data_offset} bytes) \
					don't add up to its size({size})",
					bits_per_pixel = object.info_header.bits_per_pixel,
					padding = row_size % 4,
					size = object.file_header.file_size
				)
			));
		}

		object.pixels = vec![0; width * height];

		if bytes_per_pixel == 3 {
			for index in 0..width * height {
				object.pixels[index] = Color {
					alpha: 0xFF,
					red: bytes[data_offset + index * 3],
					green: bytes[data_offset + index * 3 + 1],
					blue: bytes[data_offset + index * 3 + 2]
				}.to();
			}
		} else {
			for index in 0..width * height {
				object.pixels[index] = Color {
					alpha: bytes[data_offset + index * 4],
					red: bytes[data_offset + index * 4 + 1],
					green: bytes[data_offset + index * 4 + 2],
					blue: bytes[data_offset + index * 4 + 3]
				}.to();
			}
		}

		Ok(object)
	}
	pub fn to(self) -> SimpleImage {
		SimpleImage {
			pixels: self.pixels,
			size: Point {
				x: self.info_header.width.abs() as usize,
				y: self.info_header.height.abs() as usize
			}
		}
	}
}

impl fmt::Display for Image {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(
			formatter, 
			"Image {{\n\
			\tfile type: {type}\n\
			\tfile size: {size}\n\
			\tdata offset: {offset}\n\
			\tresolution: {width}x{height}\n\
			\tbits per pixel: {pixel_size}\n\
			}}",
			type = std::str::from_utf8(&self.file_header.file_type).unwrap(),
			size = self.file_header.file_size,
			offset = self.file_header.data_offset,
			width = self.info_header.width,
			height = self.info_header.height,
			pixel_size = self.info_header.bits_per_pixel
		)
	}
}