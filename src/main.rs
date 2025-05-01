use ls;

fn main() -> std::io::Result<()> {
	colored::control::set_override(true);
	ls::run()
}
