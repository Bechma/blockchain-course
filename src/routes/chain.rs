use std::sync::Mutex;
use actix_web::{HttpResponse, Responder, web};
use crate::blockchain::Blockchain;

pub async fn get_chain(blockchain: web::Data<Mutex<Blockchain>>) -> impl Responder {
    HttpResponse::Ok().json2(&*blockchain.lock().unwrap())
}