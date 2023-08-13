use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct Admin(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Admin, Self::Error> {
        let cookie = request.cookies().get_private("admin");

        match cookie {
            Some(cookie) => Outcome::Success(Admin(cookie.value().to_string())),
            None => Outcome::Forward(()),
        }
    }
}
