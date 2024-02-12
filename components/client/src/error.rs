use snafu::{prelude::*, Backtrace};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))] // Sets the default visibility for these context selectors
pub enum Error {
    #[snafu(context(false))]
    CryptoError { source: libsignify::Error },

    #[snafu(context(false))]
    IOError { source: std::io::Error },

    #[snafu(context(false))]
    RequestError { source: reqwest::Error },

    #[snafu(whatever, display("Error: {message}"))]
    GenericError {
        message: String,
        // Having a `source` is optional, but if it is present, it must
        // have this specific attribute and type:
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}
