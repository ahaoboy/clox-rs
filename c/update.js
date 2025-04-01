const fs = require('fs');
const path = require('path');
const files = new Set();
function isInclude(referencerDir, line) {
  line = line.trim();
  if (!line.startsWith('#include')) {
    return null;
  }
  const match = line.match(/"(.+?)"/);
  if (match) {
    const relativeFile = match[1];
    if ((relativeFile.endsWith('.c') || relativeFile.endsWith('.h'))) {
      const p = path.join(referencerDir, relativeFile);
      const txt = fs.readFileSync(p, 'utf8')
      return txt;
    }
  }
  return null;
}

function dfs(dir, txt) {
  let currentText = '';
  try {
    // const dir = path.dirname(starter);
    // const lines = fs.readFileSync(starter, 'utf8').split('\n');
    const lines = txt.split('\n');
    for (const line of lines) {
      const include = isInclude(dir, line);
      if (include) {
        if (files.has(include)) {
          continue
        }
        files.add(include)
        currentText += dfs(dir, include);
      } else {
        currentText += line + '\n';
      }
    }
  } catch (error) {
    if (error.code === 'ENOENT') {
      throw new Error(`not found: ${starter}`);
    }
    throw error;
  }
  return '\n' + currentText + '\n';
}

const TPL = `
#includes "value.h"
#includes "vm.h"
#includes "common.h"
#includes "debug.h"
#includes "scanner.h"
#includes "chunk.h"
#includes "object.h"
#includes "compiler.h"
#includes "memory.h"
#includes "table.h"
#includes "chunk.c"
#includes "compiler.c"
#includes "debug.c"
#includes "memory.c"
#includes "object.c"
#includes "scanner.c"
#includes "table.c"
#includes "value.c"
#includes "vm.c"
#include "main.c"
`.trim()

function getCode(dir, tpl) {
  return dfs(dir, tpl)
}

function main() {
  const code = getCode('c', TPL);
  fs.writeFileSync('clox-one.c', code);
}

main();