use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web};

use crate::blockchain::Blockchain;

pub async fn proof_of_work(blockchain: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    let block = blockchain.get_last_block();
    let proof = blockchain.proof_of_work();
    let prev_hash = block.get_hash()[..].iter().map(|x| format!("{:02x?}", x)).collect::<String>();
    let mined_block = (*blockchain).create_block(proof, prev_hash);
    HttpResponse::Ok().json2(mined_block)
}
