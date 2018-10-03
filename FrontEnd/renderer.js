// This file is required by the index.html file and will
// be executed in the renderer process for that window.
// All of the Node.js APIs are available in this process.

const net = require('net');

const client = net.createConnection({ path: '/tmp/rusttron.test' }, () => {
    // connect listenere
    console.log('connected to server!');
    client.write('hello server\r\n');
});

client.on('data', (data) => {
    console.log(data.toString());
    client.end();
});

client.on('end', () => {
    console.log('disconnected from server');
});
