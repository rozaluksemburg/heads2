mod app;

use leptos::*;
use app::App;
///
fn main() {
    // console_error_panic_hook::set_once(); // пока закомментировано

    mount_to_body(|| {
        view! { <App /> }
    });
}