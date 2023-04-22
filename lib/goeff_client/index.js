import init, { version_number } from  "./pkg/goeff_client.js";

async function run() {
    await init();

    const version = version_number();
    console.log("Version: " + version);
}

run();
