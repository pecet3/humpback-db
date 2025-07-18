use std::sync::Arc;

mod http_service;
mod js;
mod kv;
mod sql;
mod state;
const DIR_PATH: &str = "./humpback-data";
const STORE_PATH: &str = "./humpback-data/store.db";
fn main() {
    println!(
        r#"
        ────────────────────────────────────────────
          🐋 Humpback KV Database
          Licensed under MIT/Apache-2.0
          
          Created by Jakub Pacewicz 
          http://github.com/pecet3/humpback-kv-db
        ────────────────────────────────────────────
        "#
    );

    match http_service::run() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}
