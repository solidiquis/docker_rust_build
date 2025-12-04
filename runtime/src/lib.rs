pub fn run_future<T, F: Future<Output = T>>(op: F) -> T {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build Tokio runtime");

    runtime.block_on(op)
}
