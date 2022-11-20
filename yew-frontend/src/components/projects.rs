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
            .map(|exp| {
                html!(
                <ProjectCard project={exp}/>
                )
            })
            .collect::<Html>();

        html!(

        <div id="projects">
        {list_content}  </div>
        )
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub project: Project,
}

#[derive(Clone)]
pub struct ProjectCard {
    props: Props,
}

impl Component for ProjectCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let project = self.props.project.clone();
        html! {
                   <div key={project.id}>
                    <a href={project.link}>{project.title}</a>
                    <p>{project.description}</p>
                    </div>

        }
    }
}
