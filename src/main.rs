use std::fs::File;
use appconfig::AppConfig;
use std::env;
use kafka::consumer::{Consumer, GroupOffsetStorage, FetchOffset};
use std::io::Result;
use actix;

mod models;
mod app;
mod appconfig;
mod utils;

// fn _build_kafka_consumer(config: &app_config::AppConfig) -> Result<Consumer>{
//     let mut consumer =
//     Consumer::from_hosts(vec!("localhost:9092".to_owned()))
//        .with_topic_partitions("my-topic".to_owned(), &[0, 1])
//        .with_fallback_offset(FetchOffset::Earliest)
//        .with_group(config.server.context_path.to_owned())
//        .with_offset_storage(GroupOffsetStorage::Kafka)
//        .create()
//        .unwrap();
//        Ok(consumer)
// }

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "notif=debug,actix_web=info");
    }
    env_logger::init();
    let f = File::open("config/config.yml").expect("Config file not found.");
    println!("Opened file {:?}", f);
    let config: AppConfig = serde_yaml::from_reader(f).map_err(|err| {
        println!("error reading config file")
    }).unwrap();
    println!("Config : {:?}", config);

    let sys = actix::System::new("notif");

    app::start(&config);

    let _ = sys.run();
}