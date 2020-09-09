use actix;
use actix_web::{
    middleware, App, HttpServer, web
};

use crate::handler;

pub fn run(port: &str, web_workers: usize) {
    ////////////////////////////////////////////////////////////////////////////////
    // Create Actix
    let sys = actix::System::new("An example Protocol Buffer Server via Webserver");
    let bind = format!("0.0.0.0:{}", port);

    let _server_addr = HttpServer::new( move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::post().to_async(handler::receive_protobuf))
            )
    }).backlog(8192)
        .bind(bind.clone())
        .unwrap()
        .workers(web_workers) // By default http server uses number of available logical cpu as threads
        .keep_alive(5)
        .shutdown_timeout(30)
        .start();

    println!("Start http server: {}", bind);
    let _ = sys.run();
    println!("Stop http server: {}", bind);
}
