const { ApiPromise, WsProvider} = require('@polkadot/api');

// construct parameter for API instance 
const wsProvider = new WsProvider('ws://localhost:9944');
const rpc = {
    templateModule: {
        something_get: {
            description: "test", 
            params: [],
            type: "u32",
        }
    }
};
async function main() {
    // Construct the actual api
    const api = await ApiPromise.create({
        provider: wsProvider,
        rpc,
    });
    

    const a = await api.query.templateModule.something();
    console.log('-----------------: ', a);
}
main().catch(console.error).finally(() => process.exit());