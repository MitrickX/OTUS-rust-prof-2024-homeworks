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

add_device() {
    url="devices/add"
    data="{\"room\":\"$1\", \"device\":\"$2\"}"

    do_post
}

delete_device() {
    url="devices/delete"
    data="{\"room\":\"$1\", \"device\":\"$2\"}"

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
    add_device)
        add_device "$2" "$3"
        ;;
    delete_device)
        delete_device "$2" "$3"
        ;;
    *)
        echo "Unknown command"
        ;;
esac