#[macro_use]
extern crate horrorshow;
extern crate mime;
extern crate tide;

use diesel::prelude::*;
mod views;

pub fn create() -> tide::Server<()> {
    let mut app = tide::new();
    app.at("/").get(|_| async {
        let db = database::connection::establish();
        let boards = {
            use database::schema::boards::dsl::*;
            boards.load::<database::models::Board>(&db)
        }?;
        let resp = views::top(&boards)?;
        return Ok(tide::Response::new(tide::StatusCode::Ok)
            .body_string(resp)
            .set_mime(mime::TEXT_HTML));
    });
    app.at("/:board/").get(|req: tide::Request<()>| async move {
        let db = database::connection::establish();
        let board: String = req.param("board")?;
        let board: database::models::Board = {
            use database::schema::boards::dsl::boards;
            boards.find(board).first(&db)
        }?;
        return Ok(tide::Response::new(tide::StatusCode::Ok)
            .body_string(views::board_top(&board)?)
            .set_mime(mime::TEXT_HTML));
    });
    return app;
}
