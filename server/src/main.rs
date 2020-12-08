#![deny(warnings)]

use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /ping
    let ping = warp::path("ping").map(|| "Ping!");
    // GET /hello/from/warp  // path! macro for multiple segments
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");
    // GET /sum/:u32/:32  // path! can also handel parameters
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{}", a * b));
    // GET /:u16/times/:u16
    let add = warp::path!(u16 / u16).map(|a, b| format!("{}", a + b));
    // Combines the filters in a sort of "this and then that or that" order.
    let math = warp::path("math").and(sum.or(add));

    // Combine all filters into a api;
    let routes = warp::get().and(ping.or(hello_from_warp).or(sum).or(math));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
