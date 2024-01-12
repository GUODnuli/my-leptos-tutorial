use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button 
            on:click=move|_| { set_count.update(|n| *n += 1); }
            class:red=move || count()%2 == 1
            style="position:absolute"
            style:left=move || format!("{}px", x() + 100)
            style:backgroud-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style
        >
            "Click me: "
            {count}
        </button>
        <p>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            {count()}
        </p>
    }
}
