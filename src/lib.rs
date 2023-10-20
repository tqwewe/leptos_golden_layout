use std::{any, borrow::Cow, cell::RefCell, collections::HashMap};

use bindings::{BindableComponent, ComponentContainer, DragSource};
use js_sys::Reflect;
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
    // drag_sources: Vec<DragSource>,
    // These callback functions are stored so they can be dropped with the golden layout instance.
    _bind_cb: Closure<dyn Fn(ComponentContainer, JsValue) -> BindableComponent>,
    _unbind_cb: Closure<dyn Fn(ComponentContainer)>,
    _drag_source_cbs: Vec<Closure<dyn Fn() -> JsValue>>,
}

impl GoldenLayoutInstance {
    pub fn leak_to_window(&self, key: &str) {
        let window = web_sys::window().unwrap_throw();
        Reflect::set(&window, &JsValue::from_str(key), &self.gl).unwrap_throw();
    }

    pub fn new_drag_source<T, F>(
        &mut self,
        element: &web_sys::HtmlElement,
        title: impl Into<Cow<'static, str>>,
        state_callback: F,
    ) -> DragSource
    where
        T: Serialize,
        F: Fn() -> T + 'static,
    {
        let title: Cow<'static, str> = title.into();
        let cb = Closure::new(move || {
            let state = state_callback();
            let item_config = DragSourceComponentItemConfig {
                ty: "component",
                component_state: serde_json::to_value(state)
                    .expect_throw("could not serialize state"),
                component_type: any::type_name::<T>(),
                title: Some(&title),
            };
            item_config
                .serialize(&serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true))
                .expect_throw("could not serialize item config")
        });

        let drag_source = self.gl.new_drag_source(element, &cb);

        self._drag_source_cbs.push(cb);

        drag_source
    }
}

impl Drop for GoldenLayoutInstance {
    fn drop(&mut self) {
        self.gl.destroy();
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
                serde_json::to_value(state).expect_throw("convert state to json value");

            let title: Option<&'a str> = title.into();
            let item_config = ComponentItemConfig {
                title: title.map(Cow::Borrowed),
                component_type: Cow::Borrowed(any::type_name::<T>()),
                component_state,
                header_item_config: HeaderItemConfig {
                    item_config: ItemConfig {
                        item_type: ItemType::Component,
                        id: Some("hi".into()), // TODO: Make this ID something so we can track it
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            };

            let item_config_js_value = item_config
                .serialize(&serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true))
                .expect_throw("convert item config to js value");
            gl.gl.add_item(&item_config_js_value);
        }
    })
}

/// Mounts a component to a container element with a given component state.
pub fn mount(container: web_sys::HtmlElement, component_type_name: &str, component_state: Value) {
    COMPONENTS.with(|components| {
        let components_borrowed = components.borrow();
        let Some(component) = components_borrowed.get(component_type_name) else {
            leptos::logging::error!("Component '{component_type_name}' not registered");
            return;
        };

        let view = component(component_state);
        mount_to(container, move || view);
    });
}

