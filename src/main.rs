mod cli;
mod rpc_calls;
mod types;
mod utils;
use cli::CommandLine;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    loop {
        let should_continue = CommandLine::select_option().await.unwrap();

        if should_continue == false {
            break;
        }
    }

    Ok(())
}
