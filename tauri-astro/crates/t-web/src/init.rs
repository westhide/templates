use std::sync::LazyLock;

use nill::Nil;

pub static INIT: LazyLock<Nil> = LazyLock::new(|| {
    console_error_panic_hook::set_once();
});
