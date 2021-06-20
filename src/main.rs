use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

//for json
use bytes::buf::BufExt;
use serde_json::json;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

mod config;
mod database;
mod elk; //::query_elk; //elasticsearch fluff

async fn cacheask(find: String, dblocation: String) -> serde_json::Value {
    let dbin = database::Database {
        filelocation: dblocation,
    };
    let firsttry = dbin.get(find);
    let result: serde_json::Value =
        serde_json::from_str(std::str::from_utf8(firsttry.unwrap().unwrap().as_ref()).unwrap())
            .unwrap();
    result
}

/// Get random value from elasticsearch, using elasticsearch's built in random function
async fn api_get_random(elkinstance: String, elkindex: String) -> Result<Response<Body>> {
    let elkman = elk::Elk {
        instance: elkinstance,
        index: elkindex,
    };

    let output: serde_json::Value = elkman.random(1).await.unwrap();
    let response = match serde_json::to_string(&output) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error".into())
            .unwrap(),
    };
    Ok(response)
}

/// remove a key value from the cache
async fn api_remove_key(req: Request<Body>, dblocation: String) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    if data["key"] == json!(null) {
        data["error"] = serde_json::Value::from("no key field!");
        let json = serde_json::to_string(&data)?;
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))?;
        return Ok(response);
    }
    let removekey = &data["key"];

    let dbin = database::Database {
        filelocation: dblocation,
    }; //find(title);

    let out = dbin.remove(removekey.to_string());
    data["Result"] = serde_json::Value::from(out);
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

async fn ask(
    titl: String,
    dblocation: String,
    elkinstance: String,
    elkindex: String,
) -> serde_json::Value {
    let title: &str = titl.as_str();
    let mut dbin = database::Database {
        filelocation: dblocation,
    };
    let firsttry: String = dbin.find(title.to_string());
    if firsttry != "nope".to_string() {
        let result: serde_json::Value = serde_json::Value::String(firsttry);
        return result;
    }
    //if not find we need to
    let elkhuman = elk::Elk {
        instance: elkinstance,
        index: elkindex,
    };
    let result: serde_json::Value = elkhuman.query_elk(title.to_string(), 3).await.unwrap();
    dbin.store(title.to_string(), result.clone());
    result
}

async fn api_index() -> Result<Response<Body>> {
    let output = r#" {
        endpoints: ["/search", "/removekey", "/random"]
    }"#;

    let response = match serde_json::to_string(&output) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error".into())
            .unwrap(),
    };

    Ok(response)
}

// parse the json http input
#[allow(dead_code)]
async fn api_post_response(req: Request<Body>) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    println!("data is: {}", data);
    if data["name"] != json!(null) {
        println!("new thiungs!");
    }
    data["test"] = serde_json::Value::from("test_value");
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

async fn api_cache_check(req: Request<Body>, dblocation: String) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    if data["search"] == json!(null) {
        data["error"] = serde_json::Value::from("no search field!");
        let json = serde_json::to_string(&data)?;
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))?;
        return Ok(response);
    }
    let find = &data["search"];
    let out = cacheask(find.to_string(), dblocation).await;
    data["Result"] = serde_json::Value::from(out);
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

async fn api_search_response(
    req: Request<Body>,
    elkindex: String,
    elkinstance: String,
    dblocation: String,
) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    if data["search"] == json!(null) {
        data["error"] = serde_json::Value::from("no search field!");
        let json = serde_json::to_string(&data)?;
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))?;
        return Ok(response);
    }
    let find = &data["search"];
    let out = ask(find.to_string(), dblocation, elkinstance, elkindex).await;
    data["Result"] = serde_json::Value::from(out);
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

/*
//test response function, return a json formated string
async fn test_response() -> Result<Response<Body>> {
    /*
    curl http://127.0.0.1:1337/test
    "\n        {\n            \"test\": \"Working\",\n        }"

    */
    let data = r#"
        {
            "test": "Working",
        }"#;
    let res = match serde_json::to_string(&data) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error".into())
            .unwrap(),
    };
    Ok(res)
}
*/

#[allow(dead_code)]
async fn api_get_response() -> Result<Response<Body>> {
    let data = vec!["foo", "bar"];
    let res = match serde_json::to_string(&data) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error".into())
            .unwrap(),
    };
    Ok(res)
}

// answer the http requests
async fn answer(
    req: Request<Body>,
    elkindex: String,
    elkinstance: String,
    dblocation: String,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") | (&Method::GET, "/info") => {
            api_index().await
        }
        (&Method::GET, "/random") => api_get_random(elkinstance, elkindex).await,
        (&Method::POST, "/search") => {
            api_search_response(req, elkindex, elkinstance, dblocation.to_string()).await
        }
        //        (&Method::POST, "/json_api") => api_post_response(req).await,
        (&Method::POST, "/removekey") => api_remove_key(req, dblocation).await,
        (&Method::POST, "/cachecheck") => api_cache_check(req, dblocation).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let confpath: &str = "powerelk.toml";
    let conf = config::Config::read_file(confpath).await.unwrap();
    let addr = format!("{}:{}", conf.host.bind, conf.host.port)
        .parse()
        .unwrap();

    let new_service = make_service_fn(move |_| {
        let conf2 = conf.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                let dbloc: String = conf2.host.cachelocation.clone(); //.as_str();
                let index = conf2.host.elasticsearchindex.clone(); //.as_str();
                let ins = conf2.host.elasticsearchinstance.clone(); //.as_str();
                                                                    // Clone again to ensure that client outlives this closure.
                answer(req, index.to_string(), ins.to_string(), dbloc)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
