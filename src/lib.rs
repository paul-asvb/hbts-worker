use serde_json::json;
use worker::*;

mod utils;

use mongodb::{options::ClientOptions, sync::Client};

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/:habit/:id", |mut req, ctx| async move {
            let habit = match ctx.param("habit"){
                Some(s) => s,
                None => return Response::error("no habit param found", 405),
            };
            let id = match ctx.param("id"){
                Some(s) => s,
                None => return Response::error("no id param found", 405),
            };
             Response::ok(format!("habit: {},  id: {}",habit,id))
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/mongo", |_, _ctx| async {
            let client_options = ClientOptions::parse(
                "mongodb+srv://db-user:<password>@cluster0.qpyuzrw.mongodb.net/?retryWrites=true&w=majority",
            ).await.map_err(|_|Response::error("Bad Request", 400));
             let client = Client::with_options(client_options)?;
             let database = client.database("testDB");
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
