use yew::prelude::*;

#[function_component(RobotPanel)]
fn robot_panel() -> Html {
    html!(
        <div class="robot-panel">
            {"Robot Panel"}
        </div>
    )
}
#[function_component(App)]
fn app() -> Html {
    html!(
        <div class="app">
            <h1> <RobotPanel/> </h1>
        </div>
    )
}

fn main() {
    yew::start_app::<App>();
}
