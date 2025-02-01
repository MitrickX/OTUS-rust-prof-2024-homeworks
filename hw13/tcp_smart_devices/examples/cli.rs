use tcp_smart_devices::TcpSmartSocketClient;

const ADDR: &str = "127.0.0.1:55331";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Читаем аргументы командной строки.
    let mut cli_args = std::env::args().skip(1);
    let Some(action) = cli_args.next() else {
        return Err(String::from("No action provided, use 'on' or 'off' or 'info'").into());
    };

    println!("Performing action: '{action}'...");

    // Соединяемся с умной розеткой по tcp через клиента
    let mut client = TcpSmartSocketClient::new(ADDR)?;

    match action.as_str() {
        "on" => {
            let response = client.turn_on()?;
            println!("{}", response)
        }
        "off" => {
            let response = client.turn_off()?;
            println!("{}", response)
        }
        "info" => {
            let response = client.get_info()?;
            println!("{}", response)
        }
        _ => {
            println!("Unknown action, use 'on' or 'off' or 'info'")
        }
    }

    Ok(())
}
