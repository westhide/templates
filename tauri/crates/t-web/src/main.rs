use leptos::{mount::mount_to_body, view};
use t_web::view::Main;

// #[tokio::main(flavor = "current_thread")] async
fn main() {
    console_error_panic_hook::set_once();
    t_lib::log::init_tracing_browser_subscriber_log();
    mount_to_body(|| {
        view! {
            <Main/>
        }
    })
}
