// use auth::authorization::*;
// use auth::sanitization::*;
// use std::collections::HashMap;

// #[derive(Debug, Clone, Serialize, Deserialize)]    
// pub struct AdministratorForm {
//     pub username: String,
//     pub password: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AdministratorCookie {
//     pub userid: u32,
//     pub username: String,
//     pub display: Option<String>,
// }

// impl CookieId for AdministratorCookie
// {
//     fn cookie_id<'a>() -> & 'a str
//     {
//         "plain_acid"
//     }
// }

// impl CookieId for AdministratorForm
// {
//     fn cookie_id<'a>() -> & 'a str
//     {
//         "plain_acid"
//     }
// }

// impl AuthorizeCookie for AdministratorCookie
// {
//     fn store_cookie(&self) -> String {
//         ::serde_json::to_string(self).expect("Could not serialize")
//     }

//     #[allow(unused_variables)]
//     fn retrieve_cookie(string: String) -> Option<Self> {
//         let mut des_buf = string.clone();
//         let des: Result<AdministratorCookie, _> = ::serde_json::from_str(&mut des_buf);
//         if let Ok(cooky) = des {
//             Some(cooky)
//         } else {
//             None
//         }
//     }
// }

// impl AuthorizeForm for AdministratorForm
// {
//     type CookieType = AdministratorCookie;

//     fn authenticate(&self) -> Result<Self::CookieType, AuthFail>
//     {

//     }

//     fn new_form(user: &str, pass: &str, _extra: Option<HashMap<String, String>>) -> Self
//     {
//         AdministratorForm
//         {
//             username: user.to_string(),
//             password: pass.to_string()
//         }
//     }
// }