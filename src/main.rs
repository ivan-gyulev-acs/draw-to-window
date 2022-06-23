mod point;
mod image;
mod bmp;
mod texture;
use crate::point::{Point, Axis};
use crate::image::SimpleImage;
use crate::bmp::Color;
use crate::texture::{Rectangle, Texture};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64 * 8;
const HEIGHT: usize = 64 * 8;

// fn draw(canvas: &mut SimpleImage, draw: &Texture, rotation: f64) {
// 	let mut index = Point { x: 0.0, y: 0.0 };
// 	for y in 0..draw.position.size.y {
// 		for x in 0..draw.position.size.x {
// 			index.x = x as f64;
// 			index.y = y as f64;
// 			index.rotate(rotation.to_radians());
// 			index.x += draw.position.start.x as f64;
// 			index.y += draw.position.start.y as f64;
// 			if	index.x < 0.0 ||
// 				index.y < 0.0 ||
// 				index.x as usize > canvas.size.x ||
// 				index.x as usize > canvas.size.x {
// 				continue;
// 			}
// 			canvas[Point::new(index.x as usize, index.y as usize)] = draw.image[Point::new(
// 				draw.sprite.start.x + x * draw.sprite.size.x / draw.position.size.x,
// 				draw.sprite.start.y + y * draw.sprite.size.y / draw.position.size.y
// 			)];
// 		}
// 	}
// }

#[inline(always)]
fn point_line_distance(x: f64, y: f64, a: f64, b: f64, c: f64) -> f64 {
	(a * x + b * y + c) / (a * a + b * b).sqrt()
}

fn draw(canvas: &mut SimpleImage, draw: &Texture, rotation: f64, color: u32) {
	let a = rotation.sin();
	let b = rotation.cos();
	let mut point = Point { x: 0.0, y: 0.0 };
	for y in 0..canvas.size.y {
		for x in 0..canvas.size.x {
			point.x = 
				point_line_distance(x as f64, y as f64, a, b, -a * draw.place.center.x - b * draw.place.center.y) /
				draw.place.size.x;
			point.y =
				point_line_distance(x as f64, y as f64, -b, a, b * draw.place.center.x - a * draw.place.center.y) /
				draw.place.size.y;
			if point.x.abs() >= 0.5 || point.y.abs() >= 0.5 { continue; }
			canvas[Point{x, y}] = draw.image[Point {
				x: (draw.sprite.center.x + point.x * draw.sprite.size.x) as usize,
				y: (draw.sprite.center.y + point.y * draw.sprite.size.y) as usize
			}];
		}
	}
}

fn main() {
	let mut canvas = SimpleImage::new(
		vec![Color{red: 249, green: 159, blue: 56, alpha: 255}.to(); WIDTH * HEIGHT],
		Point{x: WIDTH,y: HEIGHT}
	).unwrap();
	let mut image: SimpleImage = bmp::Image::open("leg-movement.bmp").unwrap().to();
	image.reverse(Axis::Y);
	// let background: SimpleImage = bmp::Image::open("some.bmp").unwrap().to();
	let mut texture = Texture {
		image: &image,
		sprite: Rectangle {
			center: Point {
				x: (image.size.y as f64) / 2.0,
				y: (image.size.y as f64) / 2.0
			},
			size: Point {
				x: image.size.y as f64,
				y: image.size.y as f64
			}
		},
		place: Rectangle {
			center: Point {
				x: (WIDTH / 2) as f64,
				y: (HEIGHT / 2) as f64
			},
			size: Point {
				x: (WIDTH / 2) as f64,
				y: (HEIGHT / 2) as f64
			}
		}
	};
	let mut window = Window::new(
        "Animation",
        canvas.size.x,
		canvas.size.y,
        WindowOptions::default()
    ).unwrap();

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(6944)));
	
	let mut angle = 0.0;
	let mut timer = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
		for item in canvas.pixels.iter_mut() { *item = Color { red: 249, green: 159, blue: 56, alpha: 255 }.to(); }	
		// draw(&mut canvas, &texture, angle);
		draw(&mut canvas, &texture, angle, 0x00FF0000);
		window
			.update_with_buffer(&canvas.pixels, canvas.size.x, canvas.size.y)
			.unwrap();
		angle = (angle + 0.01 + 360.0) % 360.0;
		if timer == 0 {
			texture.sprite.center.x = (texture.sprite.center.x + texture.sprite.size.x) % (texture.image.size.x as f64);
		}
		timer = (timer + 1) % 7;
		std::thread::sleep_ms(7);
	}
}