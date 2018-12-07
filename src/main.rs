extern crate hyper;
extern crate futures;
use futures::{future, Future, Stream};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Chunk};
use hyper::body::{Payload};
use hyper::service::{/*service_fn_ok, */service_fn};


extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::ToSql;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

// const PHRASE: &str = "Hello, World!";
// job table
//      id
//      name
//      devStart
//      devEnd
//      labours -> job_labours by job_id
// job_labours table
//      id
//      job_id
//      labour_type
//      labour_info -> labour by labour_id
// labour table
//      id
//      name


fn add_job (conn: PooledConnection<SqliteConnectionManager>, job_name: &str, start_date: u32, end_date: u32) -> () {
    let mut stm = conn.prepare("INSERT INTO JOBS VALUES (:name :start_date :end_date)").unwrap();
    stm.execute_named(&[(":name", &job_name.to_string()), (":start_date", &start_date), (":end_date", &end_date)]);
}
// fn add_job (pool: &r2d2::Pool<SqliteConnectionManager>) {
//     println!("q");
//     let conn = pool.get().unwrap();
//     // let mut statement = conn.prepare("INSERT INTO COMPANY VALUES (:ID, :NAME, :AGE, :ADDRESS, :SALARY)").unwrap();
//     // let res = statement.execute_named(&[(":ID", &4i32),
//     //                          (":NAME", &String::from("John")),
//     //                          (":AGE", &18i32),
//     //                          (":ADDRESS", &String::from("Beijing")),
//     //                          (":SALARY", &30.0f64)]).unwrap();
    
//     let conn = pool.get().unwrap();
//     let mut statement = conn.prepare("INSERT INTO COMPANY VALUES (?, ?, ?, ?, ?)").unwrap();
//     let res = statement.execute(&[&6i32, &"Judy".to_string(), &19i32, &"Xian".to_string(), &35.0f64]).unwrap();
//     println!("{}", res);
// }

// fn getJobByDateRange(startDate, endDate)

// fn getJobs

// fn getLabours

// fn getJobByName

// fn setJob

// fn setLabourOfJob
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
// fn hello_world(_req: Request<Body>, pool: &r2d2::Pool<SqliteConnectionManager>) -> Result<Response<Body>, &'static str> {
//     let conn = pool.get().unwrap();
//     let mut statement = conn.prepare("SELECT * FROM COMPANY ORDER BY ID LIMIT 10").unwrap();
//     let result = match statement.query_map(&[], |row| {
//         let idx: String = row.get(1);
//         idx
//     }) {
//         Ok(str_vec) => {
//             Ok(Response::new(Body::from(
//                 str_vec.fold(String::new(), |prev, s| {
//                     format!("{} {}", prev, s.unwrap())
//                 })
//             )))
//         },
//         _ => {
//             Err("err")
//         },
//     };
//     result
// }
// rusqlite::MappedRows

struct RespondMessage {
    data: serde_json::Value,
}
impl RespondMessage {
    fn new<T: ToString> (is_ok: T, pay_load: serde_json::Value) -> RespondMessage
     {
        RespondMessage {
            data: json!({
                "ok": is_ok.to_string(),
                "payload": pay_load,
            }),
        }
    }
    fn to_string (&self) -> String {
        serde_json::to_string(&self.data).unwrap()
    }
}

// fn 

// fn add_job (job_name: String) -> Result<(), &'static str> {

// }

// fn update_job_info (job_id: u32, job_name: String, start: Date, end: Date, owner_id: u32, co_workers_id: String>) {

// }

// fn get_jobs () -> Vec<u32> {

// }
fn res<T: serde::Serialize> (data: T) -> BoxFut {
    Box::new(future::ok(Response::new(Body::from(
        RespondMessage::new("Y", json!(&data)).to_string()
    ))))
}
fn get_labours (conn: PooledConnection<SqliteConnectionManager>) -> Vec<String> {
    let mut statement = conn.prepare("SELECT * FROM COMPANY ORDER BY ID LIMIT 10").unwrap();
    let mut t: Vec<String> = vec![];
    let rows = statement.query_map(&[], |row| {
        let idx: String = row.get(1);
        idx
    }).unwrap();
    for item in rows {
        t.push(item.unwrap());
    }
    t
    // let res = RespondMessage::new("Y", json!(&t));
    // let json_str = serde_json::to_string(&t).unwrap();
    // Box::new(future::ok(Response::new(Body::from(res.to_string()))))
}
fn test (conn: PooledConnection<SqliteConnectionManager>) -> BoxFut {
    let mut statement = conn.prepare("SELECT * FROM COMPANY ORDER BY ID LIMIT 10").unwrap();
    let str_vec = statement.query_map(&[], |row| {
        let idx: String = row.get(1);
        idx
    }).unwrap();
    Box::new(future::ok(Response::new(Body::from(
        str_vec.fold(String::new(), |prev, s| {
            format!("{} {}", prev, s.unwrap())
        })
    ))))
}
fn tt (s: &str) {
    let kvs = s.split("&").collect::<Vec<&str>>();
    for item in kvs {
        let kv = item.split("&").collect::<Vec<&str>>();
        let key = kv[0];
        let value = kv[1];
    }
}
struct Form {
    fields: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CC {
    c: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BB {
    a: u32,
    b: serde_json::Value,
}

fn get_data (req: Request<Body>) -> Option<Box<dyn Stream<Item = serde_json::Value, Error = hyper::Error> + Send>> {
    let (parts, body) = req.into_parts();
    if let Some(ct) = parts.headers.get("content-type") {
        if ct.to_str().unwrap().contains("application/json") {
            Some(Box::new(body.map(|chunk| {
                serde_json::from_str(
                    String::from_utf8(
                        chunk.into_bytes().to_vec()).unwrap().as_str()).unwrap()
            })))
        } else {
            None
        }
    } else {
        None
    }
}

type BoxStream = Box<dyn Stream<Item = serde_json::Value, Error = hyper::Error> + Send>;
fn get_value <T> (stream: BoxStream) -> Box<dyn Stream<Item = Result<T, serde_json::Error>, Error = hyper::Error> + Send>
where
   for <'de> T: serde::Deserialize<'de> + serde::Serialize,
{
    Box::new(stream.map(|item| {
        serde_json::from_value::<T>(item)
    }))
}

fn route (req: Request<Body>, pool: &r2d2::Pool<SqliteConnectionManager>) -> BoxFut {
    let conn = pool.get().unwrap();
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Box::new(
                future::ok(
                    Response::new(Body::from("<script src=\"https://cdn.bootcss.com/jquery/3.3.1/jquery.js\"></script>"))))
        },
        (&Method::GET, "/test2") => res(get_labours(conn)),
        (&Method::GET, "/test") => test(conn),
        (&Method::POST, "/addJob") => {
            match get_data(req) {
                Some(data) => {
                    let stream = get_value::<BB>(data).map(|bb|{
                        println!("{:?}", &bb);
                        String::from("ww")
                    });
                    Box::new(future::ok(Response::new(Body::wrap_stream(stream))))
                },
                None => {
                    Box::new(future::ok(Response::builder()
                                        .status(400)
                                        .body(Body::from("such mime type does not valid"))
                                        .unwrap()))
                },
            }
        },
        _ => {
            Box::new(future::ok(Response::builder()
                                    .status(404)
                                    .body(Body::from("404 not found"))
                                    .unwrap()))
        },
    }
}
fn main () {

    let manager = SqliteConnectionManager::file("testDB.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3012).into();
    // for i in 0..10i32 {
    //     let pool = pool.clone();
    //     thread::spawn(move || {
    //          let conn = pool.get().unwrap();
                
    //     });
    // }
    {
        // let pool = pool.clone();
        // // thread::spawn(move || {
        //     let conn = pool.get().unwrap();
        //     let mut statement =  conn.prepare("SELECT * FROM COMPANY ORDER BY ID LIMIT 10").unwrap();
        //     let mut rows = statement.query(&[]).unwrap();
        //     while let Some(row) = rows.next() {
        //         match row {
        //             Ok(r) => {
        //                 let rr: i32 = r.get(0);
        //                 let rr1: String = r.get(1);
        //                 let rr2: i32 = r.get(2);
        //                 println!("{}", rr1);
        //                 println!("{}", rr2);
        //             },
        //             _ => {
        //                 ()
        //             },
        //         }
        //     }
        // });
    }
    // A `Service` is needed for every connection, so this
    // creates on of our `hello_world` function.
    let new_svc = move || {
        let pool = pool.clone();
        // add_job(&pool);
        // service_fn_ok converts our function into a `Service`
        service_fn(move |req| {
            route(req, &pool)
        })
    };
    
    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
