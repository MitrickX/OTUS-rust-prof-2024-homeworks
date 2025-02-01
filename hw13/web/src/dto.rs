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
pub struct DeviceRequest {
    pub room: String,
    pub device: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub house_name: String,
    pub room_name: String,
    pub devices: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DevicesListRequest {
    pub room: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DevicesListResponse {
    pub house_name: String,
    pub room_name: String,
    pub devices: Vec<String>,
}
