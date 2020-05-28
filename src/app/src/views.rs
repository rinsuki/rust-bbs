use horrorshow::helper::doctype;
use horrorshow::prelude::*;

pub fn top(boards: &std::vec::Vec<database::models::Board>) -> Result<String, horrorshow::Error> {
    return html! {
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
    .into_string();
}
