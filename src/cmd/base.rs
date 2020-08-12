use crate::error::Error;

pub trait Cmd {
    fn run(&self) -> Result<(), Error>;
}

pub fn retry_cmd(tries: u16, cmd: &dyn Cmd, before_retry: &dyn Cmd) -> Result<(), Error> {
    let result = cmd.run();
    if tries == 0 {
        return result;
    }

    match result {
        Ok(_) => result,
        Err(_) => {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            before_retry.run()?;
            retry_cmd(tries - 1, cmd, before_retry)
        }
    }
}
