use auth::authorization::*;
use std::collections::HashMap;
use rocket::{Request, Outcome};
use rocket::request::FromRequest;
use bcrypt::{verify};
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug, Clone, Serialize, Deserialize)]    
pub struct AdministratorForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}



impl CookieId for AdministratorCookie
{
    fn cookie_id<'a>() -> & 'a str
    {
        "plain_acid"
    }
}

impl CookieId for AdministratorForm
{
    fn cookie_id<'a>() -> & 'a str
    {
        "plain_acid"
    }
}

impl AuthorizeCookie for AdministratorCookie
{
    fn store_cookie(&self) -> String {
        ::serde_json::to_string(self).expect("Could not serialize")
    }

    #[allow(unused_variables)]
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<AdministratorCookie, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
    }
}

impl AuthorizeForm for AdministratorForm
{
    type CookieType = AdministratorCookie;

    fn authenticate(&self) -> Result<Self::CookieType, AuthFail>
    {
        // println!("Authenticating {} with password: {}", &self.username, &self.password);
        let hash = dotenv::var("PSWHASH").unwrap();
        // println!("hash :{}", hash);
        if &self.username == &dotenv::var("UNAME").unwrap() && verify(&self.password, &hash).unwrap() {
            Ok(
                AdministratorCookie {
                    userid: 1,
                    username: "administrator".to_string(),
                    display: Some("Administrator".to_string()),
                }
            )
        } else {
            Err(
                AuthFail::new(self.username.to_string(), "Invalid credentials".to_string())
            )
        }
    }

    fn new_form(user: &str, pass: &str, _extra: Option<HashMap<String, String>>) -> Self
    {
        AdministratorForm
        {
            username: user.to_string(),
            password: pass.to_string()
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdministratorCookie {
    type Error = ();
    
    /// The from_request inside the file defining the custom data types
    /// enables the type to be checked directly in a route as a request guard
    /// 
    /// This is not needed but highly recommended.  Otherwise you would need to use:
    /// 
    /// `#[get("/protected")] fn admin_page(admin: AuthCont<AdministratorCookie>)`
    /// 
    /// instead of:
    /// 
    /// `#[get("/protected")] fn admin_page(admin: AdministratorCookie)`
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<AdministratorCookie,Self::Error>{
        let cid = AdministratorCookie::cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = AdministratorCookie::retrieve_cookie(cookie.value().to_string()) {
                    Outcome::Success(
                        cookie_deserialized
                    )
                } else {
                    Outcome::Forward(())
                }
            },
            None => Outcome::Forward(())
        }
    }
}