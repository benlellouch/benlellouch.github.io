mod components;
mod model;

use components::experiences::ExperienceList;
use components::login_form::LoginForm;
use components::projects::ProjectList;
use model::{Experience, Login, LoginOutcome, Project};

use reqwasm::http::{Request, RequestCredentials};
use serde::de::DeserializeOwned;
use serde_json;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const URL: &str = "http://127.0.0.1:8000/";

enum Msg {
    Login(bool),
    FetchedData(Resources),
}

struct Resources {
    experiences: Vec<Experience>,
    projects: Vec<Project>,
}

struct Portfolio {
    is_logged_in: bool,
    projects: Vec<Project>,
    experiences: Vec<Experience>,
}

impl Component for Portfolio {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_logged_in: false,
            projects: vec![],
            experiences: vec![],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit_login = {
            let link = ctx.link().clone();
            Callback::from(move |data: Login| {
                let link = link.clone();
                let payload = serde_json::to_string(&data).unwrap();
                spawn_local(async move {
                    let login_endpoint = format!("{URL}/login");
                    let attempt_login: LoginOutcome = Request::post(&login_endpoint)
                        .body(&payload)
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    link.send_message(Msg::Login(attempt_login.success));
                });
            })
        };
        html! {
            <>
            <div>{if self.is_logged_in {"You are logged in"} else {"You are not logged in"} }</div>
            <div id="projects">
               <ProjectList list={self.projects.clone()}/>
            </div>
            <div id="Experiences">
                <ExperienceList list={self.experiences.clone()}/>
            </div>
            <LoginForm onsubmit={submit_login} />
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login(outcome) => {
                self.is_logged_in = outcome;
                true
            }
            Msg::FetchedData(res) => {
                self.experiences = res.experiences;
                self.projects = res.projects;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            spawn_local(async move {
                let exp_endpoint = format!("{URL}/experiences");
                let fetch_exp: Vec<Experience> = fetch_collection(&exp_endpoint).await;

                let proj_endpoint = format!("{URL}/projects");
                let fetch_proj: Vec<Project> = fetch_collection(&proj_endpoint).await;
                link.send_message(Msg::FetchedData(Resources {
                    experiences: fetch_exp,
                    projects: fetch_proj,
                }));
            });
        }
    }
}

async fn fetch_collection<T>(endpoint: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    Request::get(endpoint)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

fn main() -> () {
    yew::start_app::<Portfolio>();
}
