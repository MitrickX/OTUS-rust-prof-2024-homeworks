#!/bin/bash

do_request() {
    curl -X "${method}" -H "Content-Type: application/json" \
        -d "${data}" \
        --location "http://localhost:8080/${url}" | jq .
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

get_report() {
    local socket_name="$1"
    local socket_description="$2"
    local socket_is_on="$3"
    local socket_power="$4"
    local thermometer_name="$5"
    local thermometer_description="$6"
    local thermometer_temperature="$7"
    local socket_json="{\"name\":\"${socket_name}\", \"description\":\"${socket_description}\", \"is_on\":${socket_is_on}, \"current_power\":${socket_power}}"
    local thermometer_json="{\"name\":\"${thermometer_name}\", \"description\":\"${thermometer_description}\", \"current_temperature\":${thermometer_temperature}}"

    url="report"
    data="{\"socket\":${socket_json}, \"thermometer\":${thermometer_json}}"

    echo "${data}" ; echo 

    do_get
}

demo() {
    echo "== adding Room 1 =="
    add_room "Room 1" ; echo 

    echo "== adding Room 2 =="
    add_room "Room 2" ; echo 

    echo "== adding Socket 1 to Room 1 =="
    add_room_device "Room 1" "Socket 1" ; echo 

    echo "== adding Socket 2 to Room 1 =="
    add_room_device "Room 1" "Socket 2" ; echo 

    echo "== adding Socket 3 to Room 2 =="
    add_room_device "Room 2" "Socket 3" ; echo 

    echo "== adding Socket 4 to Room 2 =="
    add_room_device "Room 2" "Socket 4" ; echo 

    echo "== adding Thermometer 1 to Room 2 =="
    add_room_device "Room 2" "Thermometer 1" ; echo

    echo "== adding Thermometer 2 to Room 2 =="
    add_room_device "Room 2" "Thermometer 2" ; echo

    echo "== getting rooms =="
    get_rooms ; echo

    echo "== getting Room 1 devices =="
    get_room_devices "Room 1" ; echo

    echo "== getting Room 2 devices =="
    get_room_devices "Room 2" ; echo

    echo "== getting report ==";
    get_report "Socket 1" "Socket 1 description" "false" "220" "Thermometer 1" "Thermometer 1 description" "25" ; echo
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
    get_report)
        get_report "$2" "$3" "$4" "$5" "$6" "$7" "$8"
        ;;
    demo)
        demo
        ;;
    *)
        echo "Unknown command"
        ;;
esac