use std::str;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomRequest {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub house_name: String,
    pub room_name: String,
    pub devices: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomsListResponse {
    pub house_name: String,
    pub rooms: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomDeviceRequest {
    pub room: String,
    pub device: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomDeviceResponse {
    pub house_name: String,
    pub room_name: String,
    pub devices: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomDevicesListRequest {
    pub room: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomDevicesListResponse {
    pub house_name: String,
    pub room_name: String,
    pub devices: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeclareSocketRequest {
    pub name: String,
    pub description: String,
    pub is_on: bool,
    pub current_power: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeclareSocketResponse {
    pub name: String,
    pub description: String,
    pub is_on: bool,
    pub current_power: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeclareThermometerRequest {
    pub name: String,
    pub description: String,
    pub current_temperature: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeclareThermometerResponse {
    pub name: String,
    pub description: String,
    pub current_temperature: f64,
}
