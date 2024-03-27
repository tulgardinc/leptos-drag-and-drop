use leptos::html::Div;
use leptos::{logging::log, *};

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    let (is_dragging, set_is_dragging) = create_signal(false);
    let (mouse_x, set_mouse_x) = create_signal(0);
    let (mouse_y, set_mouse_y) = create_signal(0);
    let square_ref = create_node_ref();

    window_event_listener(ev::mousedown, move |e| {
        let target: web_sys::HtmlElement = event_target(&e);
        let square: HtmlElement<Div> = square_ref.get().unwrap();
        if square.contains(Some(&target)) {
            set_is_dragging.set(true);
        }
    });

    window_event_listener(ev::mousemove, move |e| {
        if is_dragging.get() {
            set_mouse_x.set(e.client_x() - 100);
            set_mouse_y.set(e.client_y() - 100);
        }
    });

    window_event_listener(ev::mouseup, move |e| {
        set_is_dragging.set(false);
    });

    view! {
        <div ref=square_ref id="square" style={move || format!("top: {}px; left: {}px", mouse_y.get(), mouse_x.get())}><h1>Drag Me!!!</h1></div>
    }
}
