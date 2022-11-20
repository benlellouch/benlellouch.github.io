use crate::model::Experience;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
    pub list: Vec<Experience>,
}

pub struct ExperienceList {}

impl Component for ExperienceList {
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
                <ExperienceCard experience={exp}/>
                )
            })
            .collect::<Html>();

        html!({ list_content })
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub experience: Experience,
}

#[derive(Clone)]
pub struct ExperienceCard {}

impl Component for ExperienceCard {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let experience = ctx.props().experience.clone();
        html! {
                   <div key={experience.id}>
                    <a href={experience.org_link}>{experience.title}</a>
                    <p>{experience.description}</p>
                    </div>

        }
    }
}
