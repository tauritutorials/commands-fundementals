import { invoke } from "@tauri-apps/api/core";

let greetMsgEl: HTMLElement | null;

async function greet() {
    let msg: string = await invoke("greet", {
        name: "Kramer",
    });

    if (greetMsgEl) {
        greetMsgEl.innerHTML += msg + "<br>";
    }
}

async function greet2() {
    let msg = await invoke("greet2", {
        firstName: "James",
        lastName: "Bond",
    });

    if (greetMsgEl) {
        greetMsgEl.innerHTML += msg + "<br>";
    }
}

async function savePreferences() {
    await invoke("save_preferences", {
        preferences: {
            firstName: "Kramer",
            theme: "dark",
        },
    });
}

async function blocking() {
    console.log(await invoke("blocking_cmd"));
}

async function non_blocking() {
    console.log(await invoke("non_blocking_cmd"));
}

async function appHandle() {
    await invoke("using_app_handle");
}

async function hide() {
    setTimeout(() => {
        invoke("hide_window");
    }, 3000);
}


async function usingState() {
    await invoke("using_state");
}


async function errors() {
    try {
        await invoke("errors");
    } catch (e) {
        console.error(e)
    }
}

window.addEventListener("DOMContentLoaded", () => {
    greetMsgEl = document.querySelector("#greet-msg");

    greet();
    // greet2();
    // savePreferences();
    // blocking();
    // non_blocking();
    // appHandle();
    // hide();
    // usingState();
    // errors();
});
