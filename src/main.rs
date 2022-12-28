// #![deny(warnings)]

use db_utility::{Album};
use serde::{Deserialize, Serialize};
use serde_json::{to_string};
use warp::Filter;

pub mod db_utility;
mod ws_router;
#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(move |socket| ws_router::user_connected(socket))
    });

    //get
    let main = warp::path::end().map(|| "Hello, World!");

    let get_header_info = warp::path!("info_header")
        //get_header
        .and(warp::header::<String>("authorization"))
        .map(|header| warp::reply::html(header));

    let test_json = warp::path!("info_json").map(|| {
        let mut employee2 = Employee {
            name: "two".to_string(),
            rate: 2222,
        };

        employee2.name = "three".to_string();
        employee2.rate = 1111;
        //json_convert
        warp::reply::json(&employee2)
    });

    let db_total_rows_async =
        warp::path!("db_total_rows")
            .and(warp::get())
            .then(|| async {
                let output = db_utility::get_total_albums().await;
                match output {
                    Ok(total) => format!("{}", total),
                    Err(err) => format!("{}", err.to_string()),
                }
            });

    let db_query_all_async = warp::path!("db_query_all")
        .and(warp::get())
        .then(|| async {
            let output = db_utility::get_list_albums().await;
            match output {
                Ok(albums) => {
                    let json1 = to_string(&albums);
                    match json1 {
                        Ok(json) => {
                            format!("{}", json)
                        }
                        Err(err) => format!("{}", err.to_string()),
                    }
                }
                Err(err) => format!("{}", err.to_string()),
            }
        });

    let db_insert_async = warp::path!("db_insert")
        .and(warp::get())
        .then(|| async {
            let result = db_utility::insert_album("title 123").await;
            match result {
                Ok(_) => format!("{}", "insert-ok"),
                Err(err) => format!("{}", err.to_string()),
            }
        });


    let db_update_async = warp::path!("db_update" / i64)
        .and(warp::post())
        .and(warp::body::json())
        .then(|id, album : Album| async move {

            let result = db_utility::update_album(id,&album.title).await;
            match result {
                Ok(_) => format!("{}", "update-ok"),
                Err(err) => format!("{}", err.to_string()),
            }
        });


    let db_delete_async = warp::path!("db_delete" / i64)
        .and(warp::get())
        .then(|id : i64| async move {
            let result = db_utility::delete_album(id).await;
            match result {
                Ok(_) => format!("{}", "delete-ok"),
                Err(err) => format!("{}", err.to_string()),
            }
        });

    //routing_id
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    //post
    let test_post = warp::post()
        .and(warp::path("test_post"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        // .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate + 100;
            employee.name = "one".to_string();
            warp::reply::json(&employee)
        });

    //post
    let test_post2 = warp::post()
        .and(warp::path("test_post_plain"))
        .map(|| warp::reply::html("Hello, world!"));

    let routes_get = warp::get().and(
        hello
            .or(main)
            .or(db_total_rows_async)
            .or(db_query_all_async)
            .or(db_insert_async)
            .or(db_delete_async)
            .or(test_json)
            .or(get_header_info)
            .or(ws_route),
    );
    let routes_post = warp::post().and(
        test_post
        .or(test_post2)
        .or(db_update_async)
        );
    let routes = routes_get.or(routes_post);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
