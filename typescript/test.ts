import CommonMeta from '.';

const result = await CommonMeta.parse("a=b;c=hello world;");
console.log(result.pairs); // { a: "b", c: "hello world" }

const isValid = await CommonMeta.validate("a=b;c=d;");
console.log(isValid); // true

const isNotValid = await CommonMeta.validate(";;=;=;;==");
console.log(isNotValid); // false
