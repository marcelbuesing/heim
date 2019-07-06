#![feature(async_await)]

use heim_common::prelude::*;
use heim_process as process;

#[runtime::main]
async fn main() -> Result<()> {
    let mut pids = process::pids();
    while let Some(pid) = pids.next().await {
        dbg!(pid?);
    }

    Ok(())
}
