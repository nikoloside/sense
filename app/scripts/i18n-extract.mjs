// 抽出界面里所有带中日文的字（0037）。输出 m0/i18n/source.json：每条一个原文，翻译表缺哪些一看便知。
// 不抽给模型的提示词：反引号模板和超过 240 字的长串都是提示词。
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const CJK = /[぀-ヿ一-鿿]/;
const set = new Set();
for (const f of ["m0/pocket.template.html", "m0/menu.html", "m0/pet.html"]) {
  const s = readFileSync(resolve(root, f), "utf8").replace(/`[^`]*`/g, "``");
  for (const m of s.matchAll(/"((?:[^"\\\n]|\\.)*)"/g)) if (CJK.test(m[1]) && m[1].length <= 240) set.add(JSON.parse('"' + m[1] + '"'));
  for (const m of s.matchAll(/>([^<>{}\n]+)</g)) if (CJK.test(m[1])) set.add(m[1].trim());
}
const list = [...set].filter(x => !/^[\s\p{P}]*$/u.test(x)).sort();
writeFileSync(resolve(root, "m0/i18n/source.json"), JSON.stringify(list, null, 1));
for (const lang of ["en", "ja"]) {
  const p = resolve(root, `m0/i18n/${lang}.json`);
  const d = existsSync(p) ? JSON.parse(readFileSync(p, "utf8")) : {};
  console.log(lang, "missing", list.filter(k => !(k in d)).length, "of", list.length);
}
