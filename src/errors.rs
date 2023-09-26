error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    foreign_links {
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
        DbErr(::sea_orm::error::DbErr);
    }
    errors {
        HttpNotFound {
            description("Http request returned Not found")
            display("Http request returned Not found")
        }
    }
}

