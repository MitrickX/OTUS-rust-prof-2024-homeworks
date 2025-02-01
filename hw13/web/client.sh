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
    *)
        echo "Unknown command"
        ;;
esac