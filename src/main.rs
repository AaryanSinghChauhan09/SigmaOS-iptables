use anyhow::Result;
use clap::Parser;
use iptables::iptables;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = iptables::Config { debug: cli.debug, ..Default::default() };
    let mut component = iptables::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-iptables running (Ctrl+C to stop)");
    tokio::runtime::Runtime::new()?.block_on(async {
        tokio::signal::ctrl_c().await.unwrap();
    });
    
    component.stop()?;
    Ok(())
}
