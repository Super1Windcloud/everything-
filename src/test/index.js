import { parse, stringify } from "smol-toml";

if (it !== undefined) {
  it("read config", () => {
    const doc = `
  [view]
  filter = true
  preview = true
  statusBar = true
  fontSize = 1024
  sortBy = "Name"
  `;
    const parsed = parse(doc);
    console.log(parsed);

    const toml = stringify(parsed);
    console.log(toml);
  });
}


