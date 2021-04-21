use oxidis::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let listener = std::net::TcpListener::bind("localhost:8000")
		.expect("Failed to bind random port.");
	run(listener)?.await
}