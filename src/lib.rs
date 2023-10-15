use std::{any, cell::RefCell, collections::HashMap, ops};

use bindings::{BindableComponent, ComponentContainer};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

pub use golden_layout::*;
pub use item_config::*;

mod bindings;
mod golden_layout;
mod item_config;

pub const GOLDEN_LAYOUT_JS: &'static str = include_str!("../golden-layout.js");
pub const GOLDEN_LAYOUT_CSS: &'static str = include_str!("../golden-layout.css");

thread_local! {
    pub static GOLDEN_LAYOUT: RefCell<Option<GoldenLayoutInstance>> = RefCell::new(None);
    static COMPONENTS: RefCell<HashMap<&'static str, Box<dyn Fn(Value) -> View>>> = RefCell::new(HashMap::new());
}

/// Golden layout instance.
pub struct GoldenLayoutInstance {
    gl: bindings::GoldenLayout,
    // These callback functions are stored so they can be dropped with the golden layout instance.
    _bind_cb: Closure<dyn Fn(ComponentContainer, JsValue) -> BindableComponent>,
    _unbind_cb: Closure<dyn Fn(ComponentContainer)>,
}

impl Drop for GoldenLayoutInstance {
    fn drop(&mut self) {
        self.gl.destroy();
    }
}

impl ops::Deref for GoldenLayoutInstance {
    type Target = bindings::GoldenLayout;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

/// Registers a component to be used with the golden layout.
pub fn register_component<T>()
where
    T: IntoView + Serialize + for<'de> Deserialize<'de>,
{
    COMPONENTS.with_borrow_mut(|components| {
        components.insert(
            any::type_name::<T>(),
            Box::new(|value| {
                let state: T =
                    serde_json::from_value(value).expect_throw("invalid component state");
                state.into_view()
            }),
        )
    });
}

/// Adds a panel to the current golden layout instance with state and a title.
pub fn add_panel<'a, T>(state: &T, title: impl Into<Option<&'a str>>)
where
    T: Serialize,
{
    GOLDEN_LAYOUT.with_borrow(|gl| {
        if let Some(gl) = gl {
            let component_state =
                serde_wasm_bindgen::to_value(state).expect_throw("convert state to js value");
            gl.add_component(any::type_name::<T>(), component_state, title.into());
        }
    })
}

/// Mounts a component to a container element with a given item config.
pub fn mount(
    container: web_sys::HtmlElement,
    component_type_name: String,
    item_config: ComponentItemConfig,
) {
    COMPONENTS.with(|components| {
        let components_borrowed = components.borrow();
        let Some(component) = components_borrowed.get(component_type_name.as_str()) else {
            leptos::logging::error!("Component '{component_type_name}' not registered");
            return;
        };

        let view = component(item_config.component_state);
        mount_to(container, move || view);
    });
}

