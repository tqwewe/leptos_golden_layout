# Leptos Golden Layout

[Leptos] integration with [Golden Layout].

[leptos]: https://github.com/leptos-rs/leptos
[golden layout]: https://github.com/golden-layout/golden-layout

This has many features missing but covers the basics for integrating leptos with golden layout.

### Example

```rust
use leptos_golden_layout::*;

#[component]
pub fn App() -> impl IntoView {
    // Register the counter component with our golden layout.
    register_component::<Counter>();

    // On the next tick, add the counter to our golden layout.
    create_effect(|_| {
        request_animation_frame(|| {
            add_panel(&Counter { initial_value: 0 }, "Counter");
        });
    });

    // Define the root for the golden layout.
    view! {
        <GoldenLayout />
    }
}

#[derive(Serialize, Deserialize)]
pub struct Counter {
    initial_value: i32,
}

impl IntoView for Counter {
    fn into_view(self) -> View {
        let (value, set_value) = create_signal(initial_value);
        let increment = move |_| set_value.update(|value| *value += 1);
        
        view! {
            <button on:click=increment>{value}</button>
        }.into_view()
    }
}
```
