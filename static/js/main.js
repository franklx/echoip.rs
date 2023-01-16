"use strict"
/* global
    HOST
    JSON_OBJ
*/

const tool = "curl -L"

const data = JSON.parse(JSON_OBJ)
delete data.json

const ui = {
    command: null,
    output: null,
    ipCheckBox: null,
    portCheckBox: null,
    portInput: null,
}

const query = {
    path: "",
    ipQuery: "",
    portQuery: "",
}

const getCompositePath = () => `${query.path}${query.portQuery}${query.ipQuery}`

function setCommand() {
    ui.command.innerText = `${tool} ${HOST}/${getCompositePath()}`
}

function changeInput(input, button) {
    query.path = input
    query.portQuery = ""
    ui.portInput.classList.add("hidden")
    switch (query.path) {
        case "json":
            output.innerText = JSON.stringify(data, null, 2)
            break
        case "city":
            output.innerText = data?.geo_info?.city
            break
        case "country":
            output.innerText = data?.geo_info?.country_name
            break
        case "country-iso":
            output.innerText = data?.geo_info?.country_iso
            break
        case "port":
            ui.portInput.classList.remove("hidden")
            query.path = "port"
            output.innerText = "{}"
            updatePort(ui.portInput.value)
            break
        case "ip":
            output.innerText = data["ip"]
            query.path = ""
            break
        case "asn":
            output.innerText = `${data?.geo_info?.asn_org} ${data?.geo_info?.asn}`
            query.path = ""
            break
        default:
            output.innerText = data[query.path]
    }
    setCommand()
    if (button) {
        for (const btn of document.querySelectorAll("button.selected")) {
            btn.classList.remove("selected")
        }
        button.classList.add("selected")
    }
}

function navigate(event) {
    window.location = getCompositePath()
}

function updatePort(port) {
    query.portQuery = `/${port}`
    setCommand()
}

function updateIP(ip) {
    query.ipQuery = `?ip=${ip}`
    changeInput("ip")
}

document.addEventListener("DOMContentLoaded", (ev) => {
    for (const key of Object.keys(ui)) {
        ui[key] = document.getElementById(key)
    }
    changeInput("ip")
})