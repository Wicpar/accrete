const { planetary_system } = require('./pkg/accrete-node');

console.log('Run accrete');
console.log('=======================');

const seed = 100;
const output = planetary_system(BigInt(seed), 1);
console.log(output)