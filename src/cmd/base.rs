use crate::error::Error;

pub trait Cmd {
    fn run(&self) -> Result<(), Error>;
}

pub fn retry_cmd(tries: u8, cmd: &dyn Cmd, before_retry: &dyn Cmd) -> Result<(), Error> {
    let mut i = 1;
    loop {
        let result = cmd.run();
        if let Ok(_) = result {
            return result;
        }

        if i >= (tries as i32) {
            return result;
        }

        std::thread::sleep(std::time::Duration::from_millis(500 + 500 * i as u64));
        before_retry.run()?;
        i += 1;
    }
}
