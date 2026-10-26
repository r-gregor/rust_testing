/* public function */
pub fn bye(name: &str) {
	println!("Hello {name}. Bye from module '{}'", get_module_name());
}

/* private function */
fn get_module_name() -> String {
	/*
	String::from("byes")
	// or ...
	return "byes".to_string();
	// or ...
	*/
	"byes".to_string()   // without ending semicolon => return
}
