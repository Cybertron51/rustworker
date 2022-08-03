use serde_json::json;
use worker::*;
use std::collections::HashMap;
use url::Url;
use std::borrow::Cow;
mod sendgrid_client;
use sendgrid_client::{EmailRecipientSender,SendgridClient};
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
 <form enctype="text/plain" action="/result/">
  <label for="email">Email:</label><br>
  <input type="email" id="email" name="email" value="Enter the recipient's email"><br>
  <label for="name">Give their name:</label><br>
  <input id="name" name="name" value="Enter the recipient's name"><br>
  <label for="inputbox">Message to send:</label><br>
  <textarea id="inputbox" name="inputbox" rows="10" cols="100">Enter your message here</textarea>
  <input type="submit" value="Submit">
</form> 
</body>"###;

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::from_html(PAGE))
	.get_async("/result/", |req, ctx| async move {
		// let url = Url::parse(req.url().unwrap());
        let url = (req.url()).unwrap();
        let mut pairs = url.query_pairs();
        let mut mappedpairs: HashMap<Cow<'_, str>, Cow<'_, str>> = pairs.collect();
        let sendgrid_api_key = ctx.var("SENDGRID_APIKEY")?.to_string();
        let sendgrid_client =SendgridClient::new(&sendgrid_api_key);
             sendgrid_client
        .send_email(
        EmailRecipientSender{// to
                         email:"derekyp9@gmail.com".to_string(),
                         name:"Derek Peng".to_string(),
        },
        EmailRecipientSender{// from
                         email:"testacc14324@gmail.com".to_string(),
                         name:"Test Account".to_string(),
        },
        EmailRecipientSender{// reply to
                         email:"testacc14324@gmail.com".to_string(),
                         name:"Test Account".to_string(),
        },
        "Test message",// subject
        "This is just a test message",// message
    )
    .await;
    return Response::ok("your message has been sent!");    
	    }
	) 

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
