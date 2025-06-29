async function main() {
  return await (await fetch("google.com")).text();
}

main();
