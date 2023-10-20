use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type GoldenLayout;

    #[wasm_bindgen(constructor)]
    pub fn new(
        container: &web_sys::HtmlElement,
        bind_component_event_handler: &Closure<
            dyn Fn(ComponentContainer, JsValue) -> BindableComponent,
        >,
        unbind_component_event_handler: &Closure<dyn Fn(ComponentContainer)>,
    ) -> GoldenLayout;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &GoldenLayout);

    #[wasm_bindgen(method, js_name = "addComponent")]
    pub fn add_component(
        this: &GoldenLayout,
        component_type: &str,
        component_state: &JsValue,
        title: Option<&str>,
    ) -> bool;

    #[wasm_bindgen(method, js_name = "addItem")]
    pub fn add_item(this: &GoldenLayout, item_config: &JsValue) -> bool;

    #[wasm_bindgen(method, setter, js_name = "resizeWithContainerAutomatically")]
    pub fn set_resize_with_container_automatically(this: &GoldenLayout, value: bool);

    #[wasm_bindgen(method, js_name = "newDragSource")]
    pub fn new_drag_source(
        this: &GoldenLayout,
        element: &web_sys::HtmlElement,
        item_config_callback: &Closure<dyn Fn() -> JsValue>,
    ) -> DragSource;

    #[wasm_bindgen(method, js_name = "removeDragSource")]
    pub fn remove_drag_source(this: &GoldenLayout, drag_source: &DragSource);
}

#[wasm_bindgen]
extern "C" {
    pub type ComponentContainer;

    #[wasm_bindgen(method, getter)]
    pub fn element(this: &ComponentContainer) -> web_sys::HtmlElement;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ResolvedComponentItemConfig, js_name = "resolveComponentTypeName")]
    pub fn resolve_component_type_name(item_config: &JsValue) -> Option<String>;
}

#[wasm_bindgen]
pub struct BindableComponent {
    // pub component: (),
    #[wasm_bindgen(js_name = "foo")]
    pub virt: bool,
}

#[wasm_bindgen]
extern "C" {
    pub type DragSource;
}

