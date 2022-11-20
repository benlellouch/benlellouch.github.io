mod components;
mod model;

use components::experiences::ExperienceList;
use components::projects::ProjectList;
use model::{Experience, Project};
use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const URL: &str = "http://127.0.0.1:8000/";

enum Msg {
    Login,
    FetchedData(Vec<Experience>, Vec<Project>),
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div id="projects">
               <ProjectList list={self.projects.clone()}/>
            </div>
            <div id="Experiences">
                <ExperienceList list={self.experiences.clone()}/>
            </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => false,
            Msg::FetchedData(exp, proj) => {
                self.experiences = exp;
                self.projects = proj;
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
                link.send_message(Msg::FetchedData(fetch_exp, fetch_proj));
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
