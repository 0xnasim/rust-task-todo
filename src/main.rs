use components::button::Button;
use components::header::Header;
use components::modal::Modal;
use components::task::task_list::TaskList;
use components::task::types::Task;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    let active_modal = use_state(|| false);
    let tasks: Vec<Task> = Vec::new();

    let on_open_modal = {
        let active_modal = active_modal.clone();
        Callback::from(move |_| active_modal.set(true))
    };

    let on_close_modal = {
        let active_modal = active_modal.clone();
        Callback::from(move |_| active_modal.set(false))
    };

    html!(
        <main class="bg-grey-100 h-screen">
            <div class="mx-auto max-w-screen-2xl">
                <Header on_click={on_open_modal} />
                <TaskList tasks={tasks.clone()} />
                <Modal active={*active_modal}>
                    <h1>{"fewfwef"}</h1>
                    <Button name="Close" on_click={on_close_modal} />
                </Modal>
            </div>
        </main>
    )
}

fn main() {
    yew::start_app::<App>();
}
