extern crate vectors;

use vectors::*;

fn main() {
	let k = Vector::new(3.0, 4.0);
	let t = Vector::new(2.0, 4.0);

	println!("{:?}", &k + &t);
	println!("{:?}", &k - &t);
	println!("{:?}", k.inverse());
	println!("{:?}", k.direction(&t));
	println!("{:?}", k.normalize());
	println!("{}", k.angle(&t));
	println!("{}", k.angle_degrees(&t));
	println!("{:?}", k.proj(&t));
	println!("{:?}", t.proj(&k));
	println!("{:?}", 2.0 * &k);
}