use alloc::string::ToString;

pub(crate) fn print_info<TNameRet: ToString, TInfoRet: ToString, TName: FnOnce() -> TNameRet, TInfo: FnOnce() -> TInfoRet>(_name: TName, _info: TInfo) {
    //eprintln!("--------------------- {} ---------------------\n", (_name()).to_string());
    //eprintln!("{}\n", (_info()).to_string());
    //eprintln!("-------------------------------------------------------------\n");
}

pub(crate) trait ExpectElseResult<T, E> {
    fn expect_else<TInfoRet: ToString, TInfo: FnOnce(&E) -> TInfoRet>(self, info: TInfo) -> T;
}

impl<T, E: core::fmt::Debug> ExpectElseResult<T, E> for Result<T, E> {
    fn expect_else<TInfoRet: ToString, TInfo: FnOnce(&E) -> TInfoRet>(self, info: TInfo) -> T {
        if self.is_ok() {
            self.expect("")
        } else {
            let error_info = match &self {
                Err(error) => info(error).to_string(),
                _ => { panic!("Unreachable point"); }
            };
            self.expect(&error_info)
        }
    }
}


pub(crate) trait ExpectElseOption<T> {
    fn expect_else<TInfoRet: ToString, TInfo: FnOnce() -> TInfoRet>(self, info: TInfo) -> T;
}

impl<T> ExpectElseOption<T> for Option<T> {
    fn expect_else<TInfoRet: ToString, TInfo: FnOnce() -> TInfoRet>(self, info: TInfo) -> T {
        if self.is_some() {
            self.expect("")
        } else {
            self.expect(&info().to_string())
        }
    }
}
