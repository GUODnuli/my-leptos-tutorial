#[allow(unused)]
use leptos::{*, ev::SubmitEvent};
use leptos::{html::Input, svg::view};
use web_sys::MouseEvent;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    // value: RwSignal<i32>,
    value: i32
}

fn main() {
    leptos::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() *2;
    let values = vec![0, 1, 2];
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters
        .map(|(count, set_count)|{
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n+= 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();
    let (control_name, set_control_name) = create_signal("Controlled".to_string());
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };
    let (value, set_value) = create_signal("B".to_string());
    let (check_value, set_check_value) = create_signal(0);
    let (option_value, set_option_value) = create_signal(0);
    let (toggled, set_toggled) = create_signal(false);
    provide_context(set_toggled);

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
        <br/>
        <ProgressBar2
            progress=Signal::derive(double_count)
        />
        <br/>
        <ProgressBar3 />
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>

        // <SizeOf<usize>/>
        // <br/>
        // <SizeOf<String>/>

        // this will just render "012"
        // <p>{values.clone()}</p>
        // or we can wrap them in <li>
        // <ul>
        //     {values.into_iter()
        //         .map(|n| view! { <li>{n}</li> } )
        //         .collect::<Vec<_>>()
        //     }
        // </ul>
        <ul>{counter_buttons}</ul>

        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
        <ComplexData/>

        <br/>

        <input type="text"
            on:input=move |ev| {
                set_control_name(event_target_value(&ev));
            }
            prop:value=control_name
        />
        <p>"Name is: " {control_name}</p>

        <br/>

        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: "{name}</p>

        <br/>

        // <UncontrolledComponent/>

        <textarea
            prop:value=control_name
            on:input=move |ev| {
                set_control_name(event_target_value(&ev));
            }
        >
            {untrack(move || control_name.get())}
        </textarea>

        <br/>

        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value(new_value);
        }>
            <SelectOption is="A" value/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>

        <br/>

        <CheckOddIf value=check_value/>
        
        <br/>

        <CheckOddOption value=option_value set_value=set_option_value/>

        <br/>

        <MatchStatements value=option_value set_value=set_option_value/>

        <br/>

        <ReturndifferentHTML value=option_value set_value=set_option_value/>

        <br/>

        <NumericInput/>

        <br/>

        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
        <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
        <ButtonC on_click=move |_| set_toggled.update(|value| *value = !*value)/>
        <ButtonD on:click=move |_| set_toggled.update(|value| *value = !*value)/>
        <Layout/>
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

#[component]
fn ProgressBar2(#[prop(default = 100)] max: u16, #[prop(into)] progress: Signal<i32>) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn ProgressBar3(#[prop(default = 100)] max: u16, #[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>) -> impl IntoView
{
    progress.map(|progress| {
        view! {
            <progress
                max=max
                value=progress
            />
        }
    })
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar4(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    /* ... */
}

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);

        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });

        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != & id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn ComplexData() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            // value: create_rw_signal(10),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            // value: create_rw_signal(20),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            // value: create_rw_signal(15),
            value: 15,
        },
    ]);

    view! {
        <button on:click=move |_| {
            set_data.update(|data| {
            // data.with(|data| {
                for row in data {
                    // row.value.update(|val| *val *= 2);
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            // each=data
            // key=|state| state.key.clone()
            // let:child
            each=move || data().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />
            // <p>{child.value}</p>
        // </For>
    }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value() == is
        >
            {is}
        </option>
    }
}

#[component]
fn CheckOddIf(value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    view! {
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>
    }
}

#[component]
fn CheckOddOption(value: ReadSignal<i32>, set_value: WriteSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    // let message = move || {
    //     if is_odd() {
    //         Some("Ding ding ding!")
    //     } else {
    //         None
    //     }
    // };

    let message = move || is_odd().then(|| "Ding ding ding!");

    view! {
        <p>{message}</p>
        <button 
            on:click=move|_| { set_value.update(|n| *n += 1); }
        >
            "Click me"
        </button>
    }
}

#[component]
fn MatchStatements(value: ReadSignal<i32>, set_value: WriteSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;
    let message = move || {
        match value() {
            0 => "Zero",
            1 => "One",
            n if is_odd() => "Odd",
            _ => "Even"
        }
    };

    view! {
        <p>{message}</p>
    }
}

#[component]
fn ReturndifferentHTML(value: ReadSignal<i32>, set_value: WriteSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    view! { 
        <main>
            {
                move || match is_odd() {
                    true if value() == 1 => {
                        view! { <pre>"One"</pre> }.into_any()
                    },
                    false if value() == 2 => {
                        view! { <pre>"Two"</pre> }.into_any()
                    },
                    _ => view! { <textarea>{value()}</textarea> }.into_any()
                }
            }
        </main>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Tyep a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div vlass="error">
                        <p>"Not a number! Errors:"</p>
                        // we can rander a list of errors as strings, if we'd like
                        <ul>
                            {
                                move || errors.get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>    
        </label>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}

#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ButtonC<F>(on_click: F) -> impl IntoView
    where F: Fn(MouseEvent) + 'static
{
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    view! {
        <button>"Toggle"</button>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonE/>
        </div>
    }
}

#[component]
pub fn ButtonE() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>()
        .expect("to have found the setter provided.");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}