use oxidis::startup::run;
use oxidis::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let config = get_configuration().expect("Failed to read configuration.");
	let address = format!("localhost:{}", config.application_port);
	let listener = std::net::TcpListener::bind(address)?;
	let connection = sqlx::SqliteConnection::connect(&config.database.connection_string())
		.await
		.expect("Failed to connect to Sqlite.");
	run(listener, connection)?.await
}