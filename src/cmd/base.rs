use crate::error::Error;

pub trait Cmd {
    fn run(&self) -> Result<(), Error>;
}

pub fn retry(tries: u16, callback: impl Fn() -> Result<(), Error>, before_retry: impl Fn() -> Result<(), Error>) -> Result<(), Error> {
    let result = callback();
    if tries == 0 {
        return result;
    }

    match result {
        Ok(_) => result,
        Err(_) => {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            before_retry()?;
            retry(tries - 1, callback, before_retry)
        }
    }
}
