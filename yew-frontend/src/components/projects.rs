use crate::model::Project;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
    pub list: Vec<Project>,
}

pub struct ProjectList {}

impl Component for ProjectList {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list_content = ctx
            .props()
            .list
            .clone()
            .into_iter()
            .map(|proj| {
                html!(
                <ProjectCard project={proj}/>
                )
            })
            .collect::<Html>();

        html!({ list_content })
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub project: Project,
}

#[derive(Clone)]
pub struct ProjectCard {}

impl Component for ProjectCard {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let project = ctx.props().project.clone();
        html! {
                   <div key={project.id}>
                    <a href={project.link}>{project.title}</a>
                    <p>{project.description}</p>
                    </div>

        }
    }
}
