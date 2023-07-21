use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Plan life!" }</h1>
            <span class="subtitle">{ "plan next week and view this week! Calendar, food menu, todo's, birtdays, have it
                all in one view" }<i class="heart" /></span>
            <ul class="list">
                <li>
                    <a href="#show" class="link">{ "show this week" }</a>
                </li>
                <li>
                    <a href="#plan" class="link">{ "plan next week" }</a>
                </li>
            </ul>
        </main>
    }
}
