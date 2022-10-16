mod model;
use yew::prelude::*;
use model::{Experience, Project};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

const URL: &str = "http://127.0.0.1:8000/";

enum Msg{
    Login,
    Fetched_data(Vec<Experience>, Vec<Project>),
}

struct Portfolio{
    is_logged_in: bool,
    projects: Vec<Project>,
    experiences: Vec<Experience>
}

impl Component for Portfolio{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        Self{
            is_logged_in: false,
            projects: vec![],
            experiences: vec![]
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        html!{
            <p>
            {format!("{:?}", self.projects)}
            </p>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Login => false,
            Msg::Fetched_data(exp,proj ) => {
                self.experiences = exp;
                self.projects = proj;
                true
            }
        }
        
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render{
            let link = ctx.link().clone();
            spawn_local( async move {
            let exp_endpoint = format!("{URL}/experiences");
            let fetch_exp: Vec<Experience> = Request::get(&exp_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            let proj_endpoint = format!("{URL}/projects");
            let fetch_proj: Vec<Project> = Request::get(&proj_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            link.send_message(Msg::Fetched_data(fetch_exp, fetch_proj));
            }
        );

        }
    }
}

fn main() -> (){
    yew::start_app::<Portfolio>();
}