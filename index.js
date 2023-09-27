const fs = require("fs");


let {Empty, TestMessage} = require("./js-proto/test_pb");


let empty = new Empty();

let testMessage = new TestMessage();

testMessage.setName("Test1");
testMessage.setId("123");

console.log(testMessage);

let binary = testMessage.serializeBinary();

console.log(binary);

fs.writeFileSync("proto-bin", binary);

let deserializeBinary = fs.readFileSync("rust-proto-bin");

