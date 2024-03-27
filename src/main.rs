use leptos::wasm_bindgen::JsCast;
use leptos::{logging::log, *};
use web_sys::HtmlElement;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    let (is_dragging, set_is_dragging) = create_signal(false);

    window_event_listener(ev::mousedown, move |e| {
        let square = document()
            .get_element_by_id("square")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let target: HtmlElement = event_target(&e);
        if square.contains(Some(&target)) {
            set_is_dragging.set(true);
        }
    });

    window_event_listener(ev::mousemove, move |e| {
        if is_dragging.get() {
            let square = document()
                .get_element_by_id("square")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            square
                .set_attribute(
                    "style",
                    format!(
                        "top: {}px; left: {}px",
                        e.client_y() - 100,
                        e.client_x() - 100
                    )
                    .as_str(),
                )
                .unwrap();
        }
    });

    window_event_listener(ev::mouseup, move |e| {
        set_is_dragging.set(false);
    });

    view! {
        <div id="square"><h1>Drag Me!!!</h1></div>
    }
}
