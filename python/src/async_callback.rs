//! Invoke a Python callback and, if it returns a coroutine and an event loop
//! is provided, schedule the coroutine on that loop (for use from non-asyncio
//! threads).

use pyo3::prelude::*;

/// If `result` is a coroutine and `event_loop` is `Some`, schedules it on the
/// loop via `asyncio.run_coroutine_threadsafe(coro, loop)`.
pub(crate) fn schedule_coro_if_needed(
    result: &Bound<PyAny>,
    event_loop: Option<&Bound<PyAny>>,
    py: Python<'_>,
) -> PyResult<()> {
    let Some(loop_ref) = event_loop else {
        return Ok(());
    };
    if result.is_none() {
        return Ok(());
    }
    let asyncio = py.import("asyncio")?;
    let is_coro = asyncio
        .getattr("iscoroutine")?
        .call1((result,))?
        .extract::<bool>()?;
    if is_coro {
        asyncio
            .getattr("run_coroutine_threadsafe")?
            .call1((result, loop_ref))?;
    }
    Ok(())
}
