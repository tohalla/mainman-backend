use actix_redis::RedisActor;
use actix_web::web::ServiceConfig;

pub fn add_cache(cfg: &mut ServiceConfig) {
    cfg.data(RedisActor::start("redis:6379"));
}
