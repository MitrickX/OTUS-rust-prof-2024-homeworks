use actix_web::{
    dev::Service,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use serde_json::json;
use smart_devices::SmartHouse;
use std::{error::Error, sync::RwLock};

pub mod dto;

type SmartHouseData = Data<RwLock<SmartHouse>>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let house = SmartHouse::new_empty("my smart house");
    let data = Data::new(RwLock::new(house));

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
            .service(add_device)
            .service(delete_device)
            .default_service(web::to(default_response))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

async fn default_response(data: SmartHouseData) -> HttpResponse {
    HttpResponse::Ok()
        .json(json! ( {"message": format!("Welcome to {}!", data.read().unwrap().name())} ))
}

#[actix_web::post("/rooms/add")]
async fn add_room(room_request: web::Json<dto::RoomRequest>, data: SmartHouseData) -> HttpResponse {
    data.write().unwrap().add_room(&room_request.name);
    HttpResponse::Ok().json(dto::RoomResponse {
        house_name: data.read().unwrap().name().to_owned(),
        room_name: room_request.name.clone(),
        devices: data.read().unwrap().devices(&room_request.name).collect(),
    })
}

#[actix_web::post("/rooms/delete")]
async fn delete_room(
    room_request: web::Json<dto::RoomRequest>,
    data: SmartHouseData,
) -> HttpResponse {
    data.write().unwrap().delete_room(&room_request.name);
    HttpResponse::Ok().json(dto::RoomsListResponse {
        house_name: data.read().unwrap().name().to_owned(),
        rooms: data.read().unwrap().rooms().collect(),
    })
}

#[actix_web::get("/rooms")]
async fn get_rooms(data: SmartHouseData) -> HttpResponse {
    HttpResponse::Ok().json(dto::RoomsListResponse {
        house_name: data.read().unwrap().name().to_owned(),
        rooms: data.read().unwrap().rooms().collect(),
    })
}

#[actix_web::post("/devices/add")]
async fn add_device(
    device_request: web::Json<dto::DeviceRequest>,
    data: SmartHouseData,
) -> HttpResponse {
    data.write()
        .unwrap()
        .add_device(&device_request.room, &device_request.device);

    HttpResponse::Ok().json(dto::DeviceResponse {
        house_name: data.read().unwrap().name().to_owned(),
        room_name: device_request.room.clone(),
        devices: data.read().unwrap().devices(&device_request.room).collect(),
    })
}

#[actix_web::post("/devices/delete")]
async fn delete_device(
    device_request: web::Json<dto::DeviceRequest>,
    data: SmartHouseData,
) -> HttpResponse {
    data.write()
        .unwrap()
        .delete_device(&device_request.room, &device_request.device);

    HttpResponse::Ok().json(dto::DeviceResponse {
        house_name: data.read().unwrap().name().to_owned(),
        room_name: device_request.room.clone(),
        devices: data.read().unwrap().devices(&device_request.room).collect(),
    })
}
