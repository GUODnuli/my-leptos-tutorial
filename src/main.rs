use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);

    view! {
        <button 
            on:click=move|_| { set_x.update(|n| *n += 10); }
            style="position:absolute"
            style:left=move || format!("{}px", x() + 100)
            style:backgroud-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            style=("--columns", x)
        >
            "Click to Move"
        </button>
        <progress
            max="50"
            value=count
        />
    }
}
