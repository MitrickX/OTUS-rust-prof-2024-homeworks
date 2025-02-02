use actix_web::{
    dev::Service,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use serde_json::json;
use smart_devices::{
    device::{info::BorrowingDeviceInfoProvider, SmartSocket, SmartThermometer},
    SmartHouse,
};
use std::{
    error::Error,
    sync::{Arc, RwLock},
};

pub mod dto;

type ArwLock<T> = Arc<RwLock<T>>;

pub struct AppState {
    pub smart_house: ArwLock<SmartHouse>,
}

type AppData = Data<AppState>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let house = SmartHouse::new_empty("my smart house");

    let data = Data::new(AppState {
        smart_house: Arc::new(RwLock::new(house)),
    });

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let addr = req.peer_addr();
                println!("Req: {req:?} from {addr:?}");
                srv.call(req)
            })
            .app_data(Data::clone(&data))
            .service(add_room)
            .service(delete_room)
            .service(get_rooms)
            .service(add_room_device)
            .service(delete_room_device)
            .service(get_room_devices)
            .service(get_report)
            .default_service(web::to(default_response))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

async fn default_response(data: AppData) -> HttpResponse {
    HttpResponse::Ok().json(
        json! ( {"message": format!("Welcome to {}!", data.smart_house.read().unwrap().name())} ),
    )
}

#[actix_web::post("/rooms/add")]
async fn add_room(room_request: web::Json<dto::RoomRequest>, data: AppData) -> HttpResponse {
    data.smart_house
        .write()
        .unwrap()
        .add_room(&room_request.name);
    HttpResponse::Ok().json(dto::RoomResponse {
        house_name: data.smart_house.read().unwrap().name().to_owned(),
        room_name: room_request.name.clone(),
        devices: data
            .smart_house
            .read()
            .unwrap()
            .devices(&room_request.name)
            .collect(),
    })
}

#[actix_web::post("/rooms/delete")]
async fn delete_room(room_request: web::Json<dto::RoomRequest>, data: AppData) -> HttpResponse {
    data.smart_house
        .write()
        .unwrap()
        .delete_room(&room_request.name);
    HttpResponse::Ok().json(dto::RoomsListResponse {
        house_name: data.smart_house.read().unwrap().name().to_owned(),
        rooms: data.smart_house.read().unwrap().rooms().collect(),
    })
}

#[actix_web::get("/rooms")]
async fn get_rooms(data: AppData) -> HttpResponse {
    HttpResponse::Ok().json(dto::RoomsListResponse {
        house_name: data.smart_house.read().unwrap().name().to_owned(),
        rooms: data.smart_house.read().unwrap().rooms().collect(),
    })
}

#[actix_web::post("/room/devices/add")]
async fn add_room_device(
    device_request: web::Json<dto::RoomDeviceRequest>,
    data: AppData,
) -> HttpResponse {
    data.smart_house
        .write()
        .unwrap()
        .add_device(&device_request.room, &device_request.device);

    HttpResponse::Ok().json(dto::RoomDeviceResponse {
        house_name: data.smart_house.read().unwrap().name().to_owned(),
        room_name: device_request.room.clone(),
        devices: data
            .smart_house
            .read()
            .unwrap()
            .devices(&device_request.room)
            .collect(),
    })
}

#[actix_web::post("/room/devices/delete")]
async fn delete_room_device(
    device_request: web::Json<dto::RoomDeviceRequest>,
    data: AppData,
) -> HttpResponse {
    data.smart_house
        .write()
        .unwrap()
        .delete_device(&device_request.room, &device_request.device);

    HttpResponse::Ok().json(dto::RoomDeviceResponse {
        house_name: data.smart_house.read().unwrap().name().to_owned(),
        room_name: device_request.room.clone(),
        devices: data
            .smart_house
            .read()
            .unwrap()
            .devices(&device_request.room)
            .collect(),
    })
}

#[actix_web::get("/room/devices")]
async fn get_room_devices(
    devices_request: web::Json<dto::RoomDevicesListRequest>,
    data: AppData,
) -> HttpResponse {
    HttpResponse::Ok().json(dto::RoomDevicesListResponse {
        house_name: data.smart_house.read().unwrap().name().to_owned(),
        room_name: devices_request.room.clone(),
        devices: data
            .smart_house
            .read()
            .unwrap()
            .devices(&devices_request.room)
            .collect(),
    })
}

#[actix_web::get("/report")]
async fn get_report(report_request: web::Json<dto::ReportRequest>, data: AppData) -> HttpResponse {
    let socket = SmartSocket::new(
        &report_request.socket.name,
        &report_request.socket.description,
        report_request.socket.is_on,
        report_request.socket.current_power,
    );

    let thermometer = SmartThermometer::new(
        &report_request.thermometer.name,
        &report_request.thermometer.description,
        report_request.thermometer.current_temperature,
    );

    let info_provider = BorrowingDeviceInfoProvider::new(&socket, &thermometer);
    let report_result = data
        .smart_house
        .read()
        .unwrap()
        .create_report(&info_provider);

    match report_result {
        Ok(report) => HttpResponse::Ok().json(dto::ReportResponse::Success(report)),
        Err(error) => HttpResponse::Ok().json(dto::ReportResponse::Error(error.to_string())),
    }
}
