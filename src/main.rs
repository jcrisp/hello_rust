#![deny(warnings)]
extern crate hyper;
extern crate url;
extern crate pretty_env_logger;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
use std::env;
use std::collections::HashMap;

fn main() {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => { port = n; },
                Err(_e) => {},
            };
        }
        Err(_e) => {},
    };
    let addr = ([0, 0, 0, 0], port).into();

    let new_service = || {
        service_fn_ok(|req| {
            let params = parse_params(req.uri());

            let mut table: u16 = 1;
            match params.get(&"table".to_string()) {
                Some(number) => {
                    match number.parse::<u16>() { 
                        Ok(n) => { table = n; },
                        Err(_e) => {},
                    }
                },
                _ => {},
            };

            let mut text = format!("{} TIMES TABLES\n", table);
            let times = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
            for &t in times.iter() {
                text.push_str(&format!("\n{} x {} = {}", t, table, t * table));
            }

            Response::new(Body::from(text))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

fn parse_params(uri: &hyper::Uri) -> HashMap<String, String> {
    let params: HashMap<String, String> = uri.query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);
    return params
}
