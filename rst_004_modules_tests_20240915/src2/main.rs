pub mod greetings;
pub mod byes;

fn main() {
	let myname: &str = "Gregor Redelonghi";
	greetings::greeting(myname);
	byes::bye(myname);

}
