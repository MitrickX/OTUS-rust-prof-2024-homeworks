use web::dto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    print!("adding Room 1: ");
    let resp = client
        .post(format!("{}{}", base_url, "/rooms/add"))
        .json(&dto::RoomRequest {
            name: "Room 1".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Room 2: ");
    let resp = client
        .post(format!("{}{}", base_url, "/rooms/add"))
        .json(&dto::RoomRequest {
            name: "Room 2".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Socket 1 to Room 1: ");
    let resp = client
        .post(format!("{}{}", base_url, "/room/devices/add"))
        .json(&dto::RoomDeviceRequest {
            device: "Socket 1".to_owned(),
            room: "Room 1".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Socket 2 to Room 1: ");
    let resp = client
        .post(format!("{}{}", base_url, "/room/devices/add"))
        .json(&dto::RoomDeviceRequest {
            device: "Socket 2".to_owned(),
            room: "Room 1".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Socket 3 to Room 2: ");
    let resp = client
        .post(format!("{}{}", base_url, "/room/devices/add"))
        .json(&dto::RoomDeviceRequest {
            device: "Socket 3".to_owned(),
            room: "Room 2".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Socket 4 to Room 2: ");
    let resp = client
        .post(format!("{}{}", base_url, "/room/devices/add"))
        .json(&dto::RoomDeviceRequest {
            device: "Socket 4".to_owned(),
            room: "Room 2".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Thermometer 1 to Room 2: ");
    let resp = client
        .post(format!("{}{}", base_url, "/room/devices/add"))
        .json(&dto::RoomDeviceRequest {
            device: "Thermometer 1".to_owned(),
            room: "Room 2".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    print!("adding Thermometer 2 to Room 2: ");
    let resp = client
        .post(format!("{}{}", base_url, "/room/devices/add"))
        .json(&dto::RoomDeviceRequest {
            device: "Thermometer 2".to_owned(),
            room: "Room 2".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("OK");

    println!();
    println!("getting rooms");
    let resp = client
        .get(format!("{}{}", base_url, "/rooms"))
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("Rooms:");
    let resp = resp.json::<dto::RoomsListResponse>().await?;
    resp.rooms.iter().for_each(|r| println!("{}", r));

    println!();
    println!("getting Room 1 devices");
    let resp = client
        .get(format!("{}{}", base_url, "/room/devices"))
        .json(&dto::RoomDevicesListRequest {
            room: "Room 1".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("Devices:");
    let resp = resp.json::<dto::RoomDevicesListResponse>().await?;
    resp.devices.iter().for_each(|d| println!("{}", d));

    println!();
    println!("getting Room 2 devices");
    let resp = client
        .get(format!("{}{}", base_url, "/room/devices"))
        .json(&dto::RoomDevicesListRequest {
            room: "Room 1".to_owned(),
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!("Devices:");
    let resp = resp.json::<dto::RoomDevicesListResponse>().await?;
    resp.devices.iter().for_each(|d| println!("{}", d));

    println!();
    println!("getting report");
    let resp = client
        .get(format!("{}{}", base_url, "/report"))
        .json(&dto::ReportRequest {
            socket: dto::SocketModel {
                name: "Socket 1".to_owned(),
                description: "Socket 1 description".to_owned(),
                is_on: false,
                current_power: 220.2,
            },
            thermometer: dto::ThermometerModel {
                name: "Thermometer 1".to_owned(),
                description: "Thermometer 1 description".to_owned(),
                current_temperature: 25.0,
            },
        })
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {:?}", resp.text().await?);
        return Ok(());
    }

    println!();
    print!("Report: ");
    let resp = resp.json::<dto::ReportResponse>().await?;
    match resp {
        dto::ReportResponse::Success(r) => println!("{}", r),
        dto::ReportResponse::Error(e) => println!("{}", e),
    }

    Ok(())
}
