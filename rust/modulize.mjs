import { readFileSync, writeFileSync, unlinkSync, cp, cpSync } from "fs";
import { join } from "path";

console.log(import.meta.url);
const dirName = "../rust/pkg/";
const content = readFileSync(dirName + "package.json");

const packageJSON = JSON.parse(String(content));
packageJSON["type"] = "module";

writeFileSync(dirName + "package.json", JSON.stringify(packageJSON));

const pkgDest = "./src/lib/pkg/";
cpSync(dirName, pkgDest, { recursive: true })
