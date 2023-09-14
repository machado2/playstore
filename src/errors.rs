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
        NotFound
        InvalidData
    }
}

pub trait OkOrError<T> {
    fn ok_or_not_found(self) -> Result<T>;
    fn ok_or_invalid(self) -> Result<T>;
}

impl<T> OkOrError<T> for Option<T> {
    fn ok_or_not_found(self) -> Result<T> {
        self.ok_or(Error::from_kind(ErrorKind::NotFound))
    }
    fn ok_or_invalid(self) -> Result<T> {
        self.ok_or(Error::from_kind(ErrorKind::InvalidData))
    }
}
