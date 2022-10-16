use rocket::{Request,Response};
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors{
    fn info(&self) -> Info{
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>)
    {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://127.0.0.1:8080"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Access-Control-Allow-Headers, Origin,Accept, X-Requested-With, Content-Type, Access-Control-Request-Method, Access-Control-Request-Headers"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

}