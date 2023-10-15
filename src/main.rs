use leptos::*;
use leptos_golden_layout::*;
use serde::{Deserialize, Serialize};

// const GOLDEN_LAYOUT_JS: &'static str = include_str!("../golden-layout.js");
// const GOLDEN_LAYOUT_CSS: &'static str = include_str!("../golden-layout.css");

fn main() {
    console_error_panic_hook::set_once();
    // leak_mount_fn();

    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();

    register_component::<Panel>();

    mount_to_body(|| view! { <App/> })

    // let app_el = document
    //     .get_element_by_id("app")
    //     .unwrap()
    //     .dyn_into()
    //     .unwrap();
    // mount_to(app_el, || view! { <App/> });
}

#[derive(Serialize, Deserialize)]
pub struct Panel {
    foo: u32,
}

impl IntoView for Panel {
    fn into_view(self) -> View {
        view! { <p>"Hey it worked! Foo is " {self.foo}</p> }.into_view()
    }
}

#[component]
fn App() -> impl IntoView {
    create_effect(|_| {
        request_animation_frame(|| {
            add_panel(&Panel { foo: 12 }, "My Panel!");
        });
    });

    view! { <GoldenLayout/> }
}

// #[component]
// fn Foo() -> impl IntoView {
//     let ctx = use_context::<i32>();
//     leptos::logging::log!("Foo {ctx:?}");

//     view! { <p>"Foo"</p> }
// }

// #[component]
// fn Bar() -> impl IntoView {
//     let ctx = use_context::<i32>();
//     leptos::logging::log!("Bar {ctx:?}");

//     view! { <p>"Bar"</p> }
// }

