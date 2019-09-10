fn main() {
	println!("I'm using the library: should be ok : {:?}", lib::addition(-3, 2).unwrap());
	println!("I'm using the library: should return an Err : {:?}", lib::addition(-3, 3));
}
