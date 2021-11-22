use actix_web::web;

use self::mine::proof_of_work;
use self::chain::get_chain;
use self::valid::is_valid;

mod mine;
mod chain;
mod valid;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/mine").route(web::get().to(proof_of_work))
    );
    cfg.service(
        web::resource("/chain").route(web::get().to(get_chain))
    );
    cfg.service(
        web::resource("/is_valid").route(web::get().to(is_valid))
    );
}
