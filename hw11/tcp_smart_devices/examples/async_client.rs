use tcp_smart_devices::asnc::client::AsyncTcpSmartSocketClient;

const ADDR: &str = "127.0.0.1:55331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut client = AsyncTcpSmartSocketClient::new(ADDR).await?;
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        match line.trim() {
            "on" => {
                let response = client.turn_on().await?;
                println!("{}", response)
            }
            "off" => {
                let response = client.turn_off().await?;
                println!("{}", response)
            }
            "info" => {
                let response = client.get_info().await?;
                println!("{}", response)
            }
            "exit" => {
                println!("bye-bye");
                break;
            }
            _ => {
                println!("Unknown action, use 'on' or 'off' or 'info'")
            }
        }
    }

    Ok(())
}
