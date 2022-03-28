const { ApiPromise, WsProvider} = require('@polkadot/api');

// construct parameter for API instance 
const wsProvider = new WsProvider('ws://localhost:9944');
const rpc = {
    kitties: {
        getKittyCnt: {
            description: "test", 
            params: [],
            type: "u64",
        }
    }
};
async function main() {
    // Construct the actual api
    const api = await ApiPromise.create({
        provider: wsProvider,
        rpc,
    });
    
    const a = await api.rpc.kitties.getKittyCnt();
    // const b = await (await api.query.templateModule.something()).toHuman();
    console.log('-----------------: ', a);  
}
main().catch(console.error).finally(() => process.exit());