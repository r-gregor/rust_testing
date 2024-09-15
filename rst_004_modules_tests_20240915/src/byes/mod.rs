/* public function */
pub fn bye(name: &str) {
	println!("Hello {name}. Bye from module '{}'", get_module_name());
}

/* private function */
fn get_module_name() -> String {
	String::from("byes")
}
