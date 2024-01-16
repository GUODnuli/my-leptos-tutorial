use leptos::*;

fn main() {
    leptos::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() *2;

    view! {
        <button 
            on:click=move|_| { set_count.update(|n| *n += 1); }
            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>
        <br/>
        <ProgressBar
            progress=count
        />
        <br/>
        <ProgressBar
            progress=double_count
        />
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}

#[component]
fn ProgressBar<F>(#[prop(default = 100)] max: u16, progress: F) -> impl IntoView
    where F: Fn() -> i32 + 'static,
{
    view! {
        <progress
            max=max
            value=progress
        />
    }
}