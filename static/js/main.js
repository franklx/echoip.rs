let data = JSON.parse(jsonObj);
delete data.json;
let tool = "curl";
let commandBox, widgetBox, compositePath, commandStr;
let path;
let ipQuery, portQuery;
let ipCheckBox, portCheckBox, portInput;
let ip = "";

window.onload = (event) => {
    commandBox = document.getElementById("command");
    widgetBox = document.getElementById("output");
    ipCheckBox = document.getElementById("ipCheckBox");
    portCheckBox = document.getElementById("portCheckBox");
    portInput = document.getElementById("portInput");
    reset();
    setcommdStr();
    changeInput("ip");
};

function reset() {
    path = "";
    ipQuery = "";
    portQuery = "";
}

function setcommdStr() {
    compositePath = `${path}${portQuery}${ipQuery}`;
    commandStr = `${tool} ${host}/${compositePath}`;
    commandBox.innerText = commandStr;
}

function changeInput(input, button) {
    // debugger;
    path = input;
    portQuery = "";
    portInput.classList.add("hidden");
    switch (path) {
        case "json":
            output.innerText = JSON.stringify(data, null, 2);
            break;
        case "city":
            output.innerText = data["geo_info"]["city"];
            break;
        case "country":
            output.innerText = data["geo_info"]["country_name"];
            break;
        case "country-iso":
            output.innerText = data["geo_info"]["country_iso"];
            break;
        case "port":
            portInput.classList.remove("hidden");
            path = "port";
            output.innerText = "{}";
            let currentPort = document.querySelector("#portInput").value;
            updatePort(currentPort);
            break;
        case "ip":
            output.innerText = data["ip"];
            path = "";
            break;
        case "asn":
            output.innerText = `${data["geo_info"]["asn_org"]} ${data["geo_info"]["asn"]}`;
            path = "";
            break;
        default:
            output.innerText = data[path];
    }
    setcommdStr();

    // set button selected
    if (button) {
        allButtons = document.querySelectorAll(("button.selected"));
        allButtons.forEach((btn) => {
            btn.classList.remove("selected");
        });

        button.classList.add("selected");
    }
}

function navigate(event) {
    console.log("navigate", compositePath);
    window.location = compositePath;
}

function updatePort(value) {
    port = value;
    portQuery = `/${port}`;
    setcommdStr();
}

function updateIP(value) {
    ip = value;
    ipQuery = `?ip=${ip}`;
    setcommdStr();
    changeInput("ip", null);
}
