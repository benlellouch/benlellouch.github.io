use std::ops::Deref;

use crate::components::custom_button::CustomButton;
use crate::components::text_input::TextInput;
use crate::model::Login;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Login>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let state = use_state(|| Login::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |password| {
        let mut data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
      <form onsubmit={onsubmit}>
        <TextInput name="Username" handle_onchange={username_changed} />
        <TextInput name="Password" handle_onchange={password_changed} />
        <CustomButton label="Submit" />
      </form>
    }
}
