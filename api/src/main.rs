use std::net::TcpListener;
use insta::app::app;
use insta::configuration::Settings;
use insta::logging::init_log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::get_configuration();
    let _g = init_log();
    let tcp_listener = TcpListener::bind(format!("{}:{}", &settings.application.host, &settings.application.port))?;

    app(tcp_listener, settings)
        .await?
        .await?;

    Ok(())
}
