mod app;

use leptos::*;
use app::{App, AppProps}; // Импортируем App и AppProps

fn main() {
    // console_error_panic_hook::set_once(); // пока закомментировано

    // Используем leptos::mount_to_body напрямую
    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}