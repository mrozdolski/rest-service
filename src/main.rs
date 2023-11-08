use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use warp::Filter;

// Define a type alias for a thread-safe HashMap
type Cache = Arc<RwLock<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    // Initialize a new Cache
    let cache: Cache = Arc::new(RwLock::new(HashMap::new()));

    // Define the "add" route
    let add = {
        let cache = cache.clone();
        warp::path!("add" / String / String)
            .and(warp::post())
            .map(move |key: String, value: String| {
                let mut cache_write = cache.write().unwrap();
                cache_write.insert(key, value);
                warp::reply::json(&"Added")
            })
    };

    // Define the "remove" route
    let remove = {
        let cache = cache.clone();
        warp::path!("remove" / String)
            .and(warp::delete())
            .map(move |key: String| {
                let mut cache_write = cache.write().unwrap();
                cache_write.remove(&key);
                warp::reply::json(&"Removed")
            })
    };

    // Define the "update" route
    let update = {
        let cache = cache.clone();
        warp::path!("update" / String / String)
            .and(warp::put())
            .map(move |key: String, value: String| {
                let mut cache_write = cache.write().unwrap();
                if cache_write.contains_key(&key) {
                    cache_write.insert(key, value);
                    warp::reply::json(&"Updated")
                } else {
                    warp::reply::json(&"Key not found")
                }
            })
    };

    // Define the "download" route
    let download = {
        let cache = cache.clone();
        warp::path!("download" / String)
            .and(warp::get())
            .map(move |key: String| {
                let cache_read = cache.read().unwrap();
                match cache_read.get(&key) {
                    Some(value) => warp::reply::json(&value),
                    None => warp::reply::json(&"Key not found"),
                }
            })
    };

    // Combine the routes
    let routes = add.or(remove).or(update).or(download);

    // Start the server
    println!("Server is listening on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
