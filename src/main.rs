use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct Todo {
    pub id: usize,
    pub contents: String,
}

#[function_component]
fn App() -> Html {
    let todos = use_state(|| vec!["Todo", "Todo2"]);

    html! {
        <main>
            <header>{"Todos"}</header>
            <ul>
                { for todos.iter().map(|todo| html! { <li>{todo}</li> }) }
            </ul>
            <footer>
                <button onclick={move |_evt| {
                    let state = todos.clone();
                    let mut todos = todos.to_vec();

                    todos.push("yeet");
                    state.set(todos);
                }}>
                    { "Add" }
                </button>
            </footer>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}