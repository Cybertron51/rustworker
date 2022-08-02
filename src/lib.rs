use serde_json::json;
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

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    static PAGE: &str = r###"<!doctype html>
<html>
<head>
	<title>Life is suffering</title>
	  <link rel="stylesheet" href="css/styles.css?v=1.0">

</head>
<body class="vsc-initialized" data-gr-ext-installed="" data-new-gr-c-s-check-loaded="14.1071.0">  
<h2>Welcome to the anonymous email sending page!</h2>
<form method="post" action="/">
  <label for="email">Enter in who you want to send the email to:</label><br>
  <input type="email" id="email" name="email" value="who you are sending to"><br>
  <label for="inputbox">and the message you want to send:</label><br>
  <textarea id="inputbox" name="inputbox" rows="10" cols="100">Enter your message here</textarea><br><br>
  <input type="submit" value="Submit">
</form> 
</body>"###;

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::from_html(PAGE))
	
        .post_async("/", |mut req, ctx| async move {
	    let form = req.form_data().await?;
            if let Some(name) = form.get("email") {
                match name {
                    FormEntry::Field(email) => {
                        let emailvalue = &email;
			return Response::ok(emailvalue)
                    }
                    FormEntry::File(_) => return Response::error("Bad Request", 400),
 
                }
            }

            Response::error("Bad Request", 400)
            
        })

        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })

        .get("/balls", |_, ctx| {
            let text = String::from("this works maybe");
            Response::ok(text)
        })

        .run(req, env)
        .await
}
