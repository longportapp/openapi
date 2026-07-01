//! Global tokio runtime shared across all contexts and language bindings.

use std::sync::LazyLock;

use tokio::runtime::Runtime;

pub(crate) static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime")
});

/// Returns a handle to the global Longport tokio runtime.
///
/// Used internally by language bindings to schedule async tasks.
#[doc(hidden)]
pub fn runtime_handle() -> tokio::runtime::Handle {
    RUNTIME.handle().clone()
}
