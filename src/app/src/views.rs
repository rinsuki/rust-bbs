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

pub fn board_top(board: &database::models::Board) -> Result<String, horrorshow::Error> {
    return html! {
        : doctype::HTML;
        html {
            head {
                meta(charset="UTF-8");
                title : format!("{} - rust-bbs", board.name);
            }
            body {
                table(width="95%", cellspacing=7, cellpadding=3, border=1, bgcolor="#CFC", align="center") {
                    tr {
                        td(style="padding: 0.5rem") {
                            h1(style="margin: 0; margin-bottom: 1rem") : board.name.to_string();
                            center : "ここに説明文を書く";
                        }
                    }
                }
            }
        }
    }
    .into_string();
}
