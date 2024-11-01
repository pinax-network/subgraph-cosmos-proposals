import block from './block.json';
// substreams run . -e mainnet.injective.streamingfast.io:443 map_blocks -s 82182057 -t 82182058 > block.json
// delete the first line of the file and last lines of the JSON file

let events = {
    block: new Set(),
    transactions: new Set(),
}

// blocks
if (block['@data'].events) {
    for (const e of block['@data'].events) {
        events.block.add(e.type);
    }
}

// transactions
for (const tx_result of block["@data"].txResults) {
    for (const e of tx_result.events) {
        events.transactions.add(e.type);
    }
}

console.log(events);