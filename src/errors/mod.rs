error_chain! {
    foreign_links {
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
        LogError(log::SetLoggerError);
        ReqError(reqwest::Error);
        ReqHeaderError(reqwest::header::ToStrError);
        SerdeJsonError(serde_json::Error);
        IntError(std::num::ParseIntError);
        IOError(std::io::Error);
        TimeError(time::OutOfRangeError);
    }

    errors {
        HTTPForbiddenError(t: String) {
            description("server returned 403 - forbidden")
            display("server returned 403 - forbidden: {}", t)
        }

        ESIErrorLimitError(t: String) {
            description("we are blocked by ESI")
            display("we are blocked by ESI: {}", t)
        }
    }
}
