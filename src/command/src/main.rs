extern crate app;
extern crate async_std;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let app = app::create();
    app.listen("0.0.0.0:3000").await
}
