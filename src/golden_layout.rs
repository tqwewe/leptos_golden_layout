use leptos::*;
use wasm_bindgen::{prelude::*, UnwrapThrowExt};

use crate::{
    bindings, golden_layout::bindings::ComponentContainer, item_config::ComponentItemConfig, mount,
    GOLDEN_LAYOUT,
};

#[component]
pub fn GoldenLayout() -> impl IntoView {
    let div_ref = create_node_ref();

    create_effect(move |_| {
        let Some(div) = div_ref.get() else {
            return;
        };

        let bind_cb = Closure::new(|container: ComponentContainer, item_config| {
            let component_type_name = bindings::resolve_component_type_name(&item_config)
                .expect_throw("missing component type name");

            let item_config: ComponentItemConfig =
                serde_wasm_bindgen::from_value(item_config).expect_throw("invalid item config");

            mount(container.element(), component_type_name, item_config);

            bindings::BindableComponent { virt: true }
        });

        let unbind_cb = Closure::new(|_container| {});

        let gl = bindings::GoldenLayout::new(
            (&*div as &web_sys::HtmlDivElement).as_ref(),
            &bind_cb,
            &unbind_cb,
        );

        gl.set_resize_with_container_automatically(true);

        GOLDEN_LAYOUT.replace(Some(crate::GoldenLayoutInstance {
            gl,
            _bind_cb: bind_cb,
            _unbind_cb: unbind_cb,
        }));
    });

    on_cleanup(|| {
        GOLDEN_LAYOUT.replace(None);
    });

    view! { <div _ref=div_ref style="width: 100%; height: 100%;"></div> }
}

