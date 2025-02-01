#!/bin/bash

do_request() {
    curl -v -X "${method}" -H "Content-Type: application/json" \
        -d "${data}" \
        --location "http://localhost:8080/${url}" ; echo 
}

do_post() {
    method="POST"
    do_request
}

do_get() {
    method="GET"
    do_request
}

add_room() {
    url="rooms/add"
    data="{\"name\":\"$1\"}"

    do_post
}

delete_room() {
    url="rooms/delete"
    data="{\"name\":\"$1\"}"

    do_post
}

get_rooms() {
    url="rooms"
    do_get
}

add_room_device() {
    url="room/devices/add"
    data="{\"room\":\"$1\", \"device\":\"$2\"}"

    do_post
}

delete_room_device() {
    url="room/devices/delete"
    data="{\"room\":\"$1\", \"device\":\"$2\"}"

    do_post
}

get_room_devices() {
    url="room/devices"
    data="{\"room\":\"$1\"}"

    do_get
}

declare_socket() {
    url="devices/socket/declare"
    data="{\"name\":\"$1\", \"description\":\"$2\", \"is_on\":$3, \"current_power\":$4}"

    do_post
}

declare_thermometer() {
    url="devices/thermometer/declare"
    data="{\"name\":\"$1\", \"description\":\"$2\", \"current_temperature\":$3}"

    do_post
}

case "$1" in
    add_room)
        add_room "$2"
        ;;
    delete_room)
        delete_room "$2"
        ;;
    get_rooms)
        get_rooms
        ;;
    add_room_device)
        add_room_device "$2" "$3"
        ;;
    delete_room_device)
        delete_room_device "$2" "$3"
        ;;
    get_room_devices)
        get_room_devices "$2"
        ;;
    declare_socket)
        declare_socket "$2" "$3" "$4" "$5"
        ;;
    declare_thermometer)
        declare_thermometer "$2" "$3" "$4"
        ;;
    *)
        echo "Unknown command"
        ;;
esac