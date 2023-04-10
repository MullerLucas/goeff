import init, { version_number } from  "./pkg/hellmut_client.js";

async function run() {
    await init();

    const version = version_number();
    console.log("Version: " + version);
}

run();
