import { createFromRoot } from "codama";
import { renderVisitor } from "@codama/renderers-rust";
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import { readFileSync } from "fs";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

// Parse command line arguments
const argv = yargs(hideBin(process.argv))
  .option("project", {
    alias: "p",
    description: "Project name",
    type: "string",
    default: "orca-whirlpool",
  })
  .help().argv;

const project = argv.project;

const pathMap = new Map();

pathMap.set("orca-whirlpool", {
  idl: "./idls/orca-whirlpool.json",
  output: "./crates/orca/whirlpools/src/generated",
});

pathMap.set("meteora-dlmm", {
  idl: "./idls/meteora-dlmm.json",
  output: "./crates/meteora/dlmm/src/generated",
});

pathMap.set("raydium-clmm", {
  idl: "./idls/raydium-clmm.json",
  output: "./crates/raydium/clmm/src/generated",
});

const metadataMap = new Map();

metadataMap.set("orca-whirlpool", {
  address: "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
  origin: "anchor",
});

metadataMap.set("meteora-dlmm", {
  address: "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
  origin: "anchor",
});

metadataMap.set("raydium-clmm", {
  address: "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
  origin: "anchor",
});

const idl = JSON.parse(readFileSync(pathMap.get(project).idl, "utf8"));
// IDL generated with anchor 0.29 does not have the metadata field so we have to add it manually
const node = rootNodeFromAnchor({
  ...idl,
  metadata: metadataMap.get(project),
});
const visitor = renderVisitor(pathMap.get(project).output);
const codama = createFromRoot(node);
codama.accept(visitor);
