#[macro_use]
extern crate horrorshow;
extern crate mime;
extern crate tide;

use diesel::prelude::RunQueryDsl;
use horrorshow::helper::doctype;
use horrorshow::prelude::*;

pub fn create() -> tide::Server<()> {
    let mut app = tide::new();
    app.at("/").get(|_| async {
        let db = database::connection::establish();
        let boards = {
            use database::schema::boards::dsl::*;
            boards.load::<database::models::Board>(&db)
        }?;
        let resp = html! {
            : doctype::HTML;
            html {
                head {
                    meta(charset="UTF-8");
                    title : "rust-bbs";
                }
                body {
                    h1 : "rust-bbs";
                    : "Rust で書かれた BBS です";
                    h2 : "板一覧";
                    ul {
                        @ for board in boards.into_iter() {
                            li {
                                a(href = format_args!("/{}/", board.id)) : format_args!("{}", board.name)
                            }
                        }
                    }
                }
            }
        }
        .into_string()?;
        return Ok(tide::Response::new(tide::StatusCode::Ok)
            .body_string(resp)
            .set_mime(mime::TEXT_HTML));
    });
    return app;
}
