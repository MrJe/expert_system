use super::*;

fn is_file_open_working() {
	assert_eq!(file::open().unwrap(), true, "");
}

fn is_file_read_working() {
	assert_eq!(file::read().unwrap(), true, "");
}

fn is_file_print_working() {
}

#[test]
#[should_panic]
fn is_file_working([paths]) {
	for 
	is_file_open_working();
	is_file_read_working();
	is_file_print_working();
}
