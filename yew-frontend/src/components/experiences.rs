use crate::model::Experience;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
    pub list: Vec<Experience>,
}

pub struct ExperienceList {
    props: ListProps,
}

impl Component for ExperienceList {
    type Message = ();
    type Properties = ListProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list_content = self
            .props
            .list
            .clone()
            .into_iter()
            .map(|exp| {
                html!(
                <ExperienceCard experience={exp}/>
                )
            })
            .collect::<Html>();

        html!(
        <div id="experiences"> {list_content}  </div>
        )
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub experience: Experience,
}

#[derive(Clone)]
pub struct ExperienceCard {
    props: Props,
}

impl Component for ExperienceCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let experience = self.props.experience.clone();
        html! {
                   <div key={experience.id}>
                    <a href={experience.org_link}>{experience.title}</a>
                    <p>{experience.description}</p>
                    </div>

        }
    }
}
