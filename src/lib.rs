#[cfg(test)]
mod tests {
	use super::*;

	fn useless_non_tested() {
		assert_eq!(addition(8, 10).unwrap(), 18, "is addition result right");
	}

	#[test]
	fn is_addition_equal() {
		useless_non_tested();
	}

	#[test]
	#[should_panic]
	fn is_addition_zero() {
		assert_eq!(addition(-9, 9).is_err(), true, "addition should return an error");
	}
}

pub fn addition(a: i8, b: i8) -> Result<i8,  i8> {
	if a + b != 0 {
		Ok(a + b)
	} else {
		println!("ERROR !");
		Err(0)
	}
}
