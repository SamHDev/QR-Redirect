async function fetchState() {
    let rq = await fetch("/api/state", {
        credentials: "include",
        method: "GET"
    });

    return await rq.json();
}

async function fetchActive() {
    let rq = await fetch("/api/active", {
        credentials: "include",
        method: "GET"
    });

    return await rq.json();
}

async function writeCustom(value) {
    await fetch("/api/custom", {
        credentials: "include",
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(value)
    });
}

async function setActive(identifier) {
    await fetch("/api/set", {
        credentials: "include",
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(identifier)
    });
}

const url_regex = new RegExp(/[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)?/gi);

let items_container = window.items_container;
let items_map = {};

function updateItems(state) {
    items_container.innerHTML = "";
    items_map = {}

    for (let item of state["items"]) {
        let el = document.createElement("div");
        el.innerText = item["label"];
        el.setAttribute("identifier", item["identifier"]);

        if (state["active"] === item["identifier"]) {
            el.classList.add("active");
        }

        items_container.appendChild(el);
        items_map[item["identifier"]] = el;

        el.addEventListener("click", async () => {
            await setActive(item["identifier"]);
            updateActiveElement(item["identifier"]);
        });
    }

    let custom_state = state["custom"] || {};

    if (custom_state["enabled"]) {
        let el = document.createElement("div");
        el.innerText = "Custom URL";
        el.classList.add("expand");
        items_container.appendChild(el);
        items_map["@custom"] = el;

        if (state["active"] === "@custom") {
            el.classList.add("active");
        }

        el.addEventListener("click", async () => {
            await setActive("@custom");
            updateActiveElement("@custom");
        });

        el.addEventListener("dblclick", async () => {
            let new_value = prompt("Enter Custom URL", custom_state["value"] || undefined);
            if (!new_value.match(url_regex)) {
                alert("Invalid CUSTOM URL");
                return;
            }

            custom_state["value"] = new_value;
            await writeCustom(new_value);
        })
    }
}

function updateActiveElement(identifier) {
    for (let key of Object.keys(items_map)) {
        if (key !== identifier) {
            items_map[key].classList.remove("active");
        } else {
            items_map[key].classList.add("active");
        }
    }
}

window.onload = async () => {
    items_container = window.items_container;

    let state = await fetchState();
    updateItems(state);

    setInterval(async() => {
        let active = await fetchActive();
        updateActiveElement(active);
    }, 5000);
}