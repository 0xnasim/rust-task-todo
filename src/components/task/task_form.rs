use yew::{
    function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties,
};

use crate::components::task::types::Task;

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub on_add: Callback<Task>,
}

#[function_component(TaskForm)]
pub fn task_form() -> Html {
    let title = use_state(|| "".to_string());
    let description = use_state(|| "".to_string());

    let oninput = {
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let value = e.data();
        })
    };

    html! {
        <form>
            {"fegqufgug"}
        </form>
    }
}
