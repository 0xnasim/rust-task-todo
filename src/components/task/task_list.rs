use crate::components::task::task_item::TaskItem;
use crate::components::task::types::Task;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
}

#[function_component(TaskList)]
pub fn task_list(props: &TaskListProps) -> Html {
    html!(
    <div class="px-10 flex justify-between flex-wrap">
        {props.tasks.iter().map(|task| html!{
            <TaskItem item={task.clone()}/>
        }).collect::<Html>()}
    </div>
    )
}
