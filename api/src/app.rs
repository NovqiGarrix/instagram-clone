use crate::configuration::set_testing_env;
use crate::{AppState, db::connect_db};
use actix_cors::Cors;
use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use tracing::{info, instrument};
use tracing_actix_web::TracingLogger;
use crate::configuration::Settings;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "code": 200 }))
}

#[instrument(name = "App", skip(listener, config))]
pub async fn app(listener: TcpListener, config: Settings) -> Result<Server, std::io::Error> {
    let db = connect_db(&config)
        .await
        .expect("Failed while connect to DB.");

    info!("Server started at http://{}:{}", &config.application.host, &config.application.port);

    let app_state = AppState {
        db,
        config,
    };

    let app = HttpServer::new(move || {
        let cors = Cors::default().allow_any_method().allowed_headers(vec![
            "Accept",
            "Content-Type",
            "Accept-Encoding",
            "Origin",
        ]);

        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(hello))
            // .service(web::scope("/api/v1").configure(app_v1_handlers))
    })
        .listen(listener)?
        .run();

    Ok(app)
}

#[derive(Clone)]
pub struct MyTestServer {
    pub address: String,
}

pub async fn start_test_server() -> MyTestServer {
    set_testing_env();

    let host = String::from("127.0.0.1");
    let listener =
        TcpListener::bind(format!("{}:0", &host.to_owned())).expect("Failed to bind TCP Listener");

    let port = listener.local_addr().unwrap().port();
    let address = format!("{}:{}", host, port);

    let config = Settings::get_configuration();

    let server = app(listener, config)
        .await
        .expect("Failed to get the server: ");

    actix_web::rt::spawn(server);

    MyTestServer { address }
}