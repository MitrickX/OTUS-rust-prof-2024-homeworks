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
pub struct SocketModel {
    pub name: String,
    pub description: String,
    pub is_on: bool,
    pub current_power: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ThermometerModel {
    pub name: String,
    pub description: String,
    pub current_temperature: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub socket: SocketModel,
    pub thermometer: ThermometerModel,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ReportResponse {
    #[serde(rename = "report")]
    Success(String),
    #[serde(rename = "error")]
    Error(String),
}
