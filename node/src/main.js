import { hash } from "crypto";

function main() {
  const input = "helloworld";
  const start = performance.now();
  console.log(hash("md5", input));
  console.log(`${(performance.now() - start) * 1000000} ns`);
}

main();
