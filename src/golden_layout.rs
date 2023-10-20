use leptos::*;
use serde_json::Value;
use wasm_bindgen::{prelude::*, UnwrapThrowExt};

use crate::{
    bindings, golden_layout::bindings::ComponentContainer,
    item_config::ResolvedComponentItemConfig, mount, GoldenLayoutInstance, GOLDEN_LAYOUT,
};

#[component]
pub fn GoldenLayout(
    #[prop(optional)] on_ready: Option<Box<dyn Fn(&mut GoldenLayoutInstance)>>,
    #[prop(optional, into)] on_mount: Option<Callback<(web_sys::HtmlElement, String)>>,
) -> impl IntoView {
    let div_ref = create_node_ref();

    create_effect(move |_| {
        let Some(div) = div_ref.get() else {
            return;
        };

        let on_mount_cb = on_mount.clone();
        let bind_cb = Closure::new(move |container: ComponentContainer, item_config| {
            let component_type_name = bindings::resolve_component_type_name(&item_config)
                .expect_throw("missing component type name");

            let item_config_value: Value = serde_wasm_bindgen::from_value(item_config).unwrap();
            let item_config: ResolvedComponentItemConfig =
                serde_json::from_value(item_config_value)
                    .expect_throw("failed to parse item config");

            mount(
                container.element(),
                &component_type_name,
                item_config.component_state,
            );

            if let Some(cb) = &on_mount_cb {
                cb.call((container.element(), component_type_name));
            }

            bindings::BindableComponent { virt: true }
        });

        let unbind_cb = Closure::new(|_container| {});

        let gl = bindings::GoldenLayout::new(
            (&*div as &web_sys::HtmlDivElement).as_ref(),
            &bind_cb,
            &unbind_cb,
        );

        gl.set_resize_with_container_automatically(true);

        let mut gl = GoldenLayoutInstance {
            gl,
            _bind_cb: bind_cb,
            _unbind_cb: unbind_cb,
            _drag_source_cbs: Vec::new(),
        };

        if let Some(cb) = &on_ready {
            cb(&mut gl);
        }

        GOLDEN_LAYOUT.replace(Some(gl));
    });

    on_cleanup(|| {
        GOLDEN_LAYOUT.replace(None);
    });

    view! { <div _ref=div_ref style="width: 100%; height: 100%;"></div> }
}

