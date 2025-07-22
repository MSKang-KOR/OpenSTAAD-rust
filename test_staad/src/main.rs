mod staad;
use staad::process::StaadProcess;

#[tokio::main]
async fn main() {
    // StaadBackgroundAnalyzer
    println!("Hello, world!");
    let mut _staad = StaadProcess::new("");
    match _staad.start().await {
        Err(e) => {
            println!("{:#?}", e)
        }
        _ => {
            let _ = _staad.test_code();
        }
    };
}
