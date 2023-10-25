use dioxus::prelude::*;
use dioxus_websocket_hooks::{use_ws_context, use_ws_context_provider_text};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    // let response = use_ref(cx, || Vec::new());
    use_shared_state_provider(&cx, || Vec::<String>::new());
    let set_response = use_shared_state::<Vec<String>>(&cx).unwrap();

    let set_response = set_response.clone();
    use_ws_context_provider_text(&cx, "wss://echo.websocket.events", move |msg| {
        set_response.write().push(msg);
    });

    cx.render(rsx!(ResponseDisplay {}))
}

#[allow(non_snake_case)]
fn ResponseDisplay(cx: Scope) -> Element {
    let response = use_shared_state::<Vec<String>>(&cx).unwrap();
    let ws = use_ws_context(&cx);

    let input = use_state(&cx, String::default);
    let submit = move |_| {
        ws.send_text(input.to_string());
        input.modify(|_| String::default());
    };

    let response = response.read();
    let response_rendered = response
        .iter()
        .map(|comment: &String| rsx!(p { "{comment}" }));

    cx.render(rsx! (
        div { "Server sent:" response_rendered }
        input { oninput: move |event| input.modify(|_| event.value.clone())  }
        button { onclick: submit, "Submit" }
    ))
}
