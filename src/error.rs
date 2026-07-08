/// Font loading operation error.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// System font loading is not available on this platform.
    #[error("system font loading not supported on this platform: {0}")]
    NotSupported(&'static str),

    /// Browser Local Font Access API error (WASM).
    #[error("web font access error: {0}")]
    Web(String),

    /// Android NDK system font loading error.
    #[error("android font loading error: {0}")]
    Android(String),

    /// Platform-specific error.
    #[error("{context}: {source}")]
    Platform {
        /// Context describing where the error occurred.
        context: &'static str,
        /// The underlying error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
