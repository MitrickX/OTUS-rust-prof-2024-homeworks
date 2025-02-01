use actix_web::{
    dev::Service,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use serde_json::json;
use smart_devices::{
    device::{SmartSocket, SmartThermometer},
    SmartHouse,
};
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

pub mod dto;

type ArwLock<T> = Arc<RwLock<T>>;

pub struct AppState {
    pub smart_house: ArwLock<SmartHouse>,
    pub declared_sockets: ArwLock<HashMap<String, SmartSocket>>,
    pub declared_thermometers: ArwLock<HashMap<String, SmartThermometer>>,
}
type AppData = Data<AppState>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let house = SmartHouse::new_empty("my smart house");

    let data = Data::new(AppState {
        smart_house: Arc::new(RwLock::new(house)),
        declared_sockets: Arc::new(RwLock::new(HashMap::new())),
        declared_thermometers: Arc::new(RwLock::new(HashMap::new())),
    });

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let addr = req.peer_addr();
                println!("From middleware fn: Hello {addr:?}");
                srv.call(req)
            })
            .app_data(Data::clone(&data))
            .service(add_room)
            .service(delete_room)
            .service(get_rooms)
            .service(add_room_device)
            .service(delete_room_device)
            .service(get_room_devices)
            .service(declare_socket)
            .service(declare_thermometer)
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

#[actix_web::post("/devices/socket/declare")]
async fn declare_socket(
    declare_socket_request: web::Json<dto::DeclareSocketRequest>,
    data: AppData,
) -> HttpResponse {
    let socket = SmartSocket::new(
        &declare_socket_request.name,
        &declare_socket_request.description,
        declare_socket_request.is_on,
        declare_socket_request.current_power,
    );
    data.declared_sockets
        .write()
        .unwrap()
        .insert(socket.name().to_owned(), socket);

    HttpResponse::Ok().json(dto::DeclareSocketResponse {
        name: declare_socket_request.name.clone(),
        description: declare_socket_request.description.clone(),
        is_on: declare_socket_request.is_on,
        current_power: declare_socket_request.current_power,
    })
}

#[actix_web::post("/devices/thermometer/declare")]
async fn declare_thermometer(
    declare_thermometer_request: web::Json<dto::DeclareThermometerRequest>,
    data: AppData,
) -> HttpResponse {
    let thermometer = SmartThermometer::new(
        &declare_thermometer_request.name,
        &declare_thermometer_request.description,
        declare_thermometer_request.current_temperature,
    );
    data.declared_thermometers
        .write()
        .unwrap()
        .insert(thermometer.name().to_owned(), thermometer);

    HttpResponse::Ok().json(dto::DeclareThermometerResponse {
        name: declare_thermometer_request.name.clone(),
        description: declare_thermometer_request.description.clone(),
        current_temperature: declare_thermometer_request.current_temperature,
    })
}

// TODO: Report methhod
// Take a list of device names, find among declared devices
// If device not declared then delcare it with description "Undeclared device"
// Then build a report using selected declared devices
