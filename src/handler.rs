use actix_web::{
    HttpResponse, Error as AWError,
};

use futures::{
    future::{ok as fut_ok, err, Either},
    Future
};

use serde_json::{to_string};
use crate::test::{Character};
use bytes::Bytes;

use protobuf::{parse_from_bytes};

pub fn receive_protobuf(body: Bytes) -> impl Future<Item = HttpResponse, Error = AWError> {
    let character = protobuf::parse_from_bytes::<Character>(&body).unwrap();
    let dumped = serde_json::to_string(&character).unwrap();

    println!(dumped);

    println!("{}", body.len());
    fut_ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(dumped))
}
