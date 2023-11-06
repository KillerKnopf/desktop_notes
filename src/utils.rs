#[cfg(debug_assertions)]
pub fn initialize_logging() {
    // Includes Backtrace feature when running the programm
    // Backtrace shows call stack when panic!
    // 0 = disabled (no backtrace)
    // 1 = partial call stack
    // full = full call stack
    std::env::set_var("RUST_BACKTRACE", "0");
    // env::set_var("RUST_BACKTRACE", "1");
    // env::set_var("RUST_BACKTRACE", "full");

    // Sets-up eyre to generate colorful reports on any panic
    color_eyre::install().expect("Failed to initalize color_eyre");

    // Setting up Tracing
    // Builder for creating a Subscriber instance
    // A Subscriber is used by Tracing to collect data and log it (e.g. to standard output)
    // First a formatter is specified
    // Then the minimum alert level a debug statement should have to be printed
    // The level represents the verbosity of an statement
    // Available Levels:
    // most verbose (lots of information) > least verbose (little information)
    // TRACE > DEBUG > INFO > Warn > Error
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    // Set defaults for the subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to initialize default subscriber");
}
