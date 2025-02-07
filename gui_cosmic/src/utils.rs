pub fn coalesce_result<T>(result: Result<T, T>) -> T {
    match result {
        Ok(ok) => ok,
        Err(err) => err,
    }
}
