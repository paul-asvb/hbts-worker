use worker::*;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

struct bla {
    name: String,
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
        .get_async("/habits", |_req, _ctx| async {
            let json = serde_json::json!({
            "habits":[
                "cigarettes","jogging","alcohol","gym","drugs"
            ]});
            Response::from_json(&json)
        })
        .post_async("/habits/:habit/:id", |_req, ctx| async move {
            let habit = match ctx.param("habit") {
                Some(s) => s,
                None => return Response::error("no habit param found", 405),
            };
            let id = match ctx.param("id") {
                Some(s) => s,
                None => return Response::error("no id param found", 405),
            };
            Response::ok(format!("habit: {},  id: {}", habit, id))
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/mongo", |_, _ctx| async {
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
