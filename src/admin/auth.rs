// use actix_web::{dev, Error, HttpRequest, FromRequest, Result};
// use actix_identity::Identity;
// use actix_web::error::ErrorBadRequest;
// use dotenv::dotenv;
// use futures::future::{ok, err, Ready};
// use serde_derive::Serialize;
// use std::{env, ops::Deref};

// #[derive(Debug)]
// pub struct User {
//     id: Identity
// }

// impl FromRequest for User {
//     type Error = Error;
//     type Future = Ready<Result<Self, Self::Error>>;
//     type Config = ();

//     fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        
//         req.identity().unwrap_or_else(|| "Anonymous".to_owned())
//     }
// }
