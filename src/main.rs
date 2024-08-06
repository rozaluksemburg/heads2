mod app;

use leptos::*;
// use console_error_panic_hook;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello, world!"</p>
    }
}

fn main() {
    // console_error_panic_hook::set_once(); // пока закомментировано
    mount_to_body(|cx| view! { cx, <App /> }); // здесь можно и Layout 
}