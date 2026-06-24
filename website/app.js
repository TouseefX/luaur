// luaur playground — boots the WebAssembly engine and wires the editor + buttons.
//
// The Luau compiler, VM and type checker are compiled to wasm (crates/luaur-web,
// `wasm` feature) and exposed as two functions:
//   run(source)   -> captured print output + any runtime error
//   check(source) -> type-checker diagnostics, or "No errors."
//
// CodeMirror 6 (with the legacy Lua mode) provides the editor. Everything runs
// client-side; there is no server.

import { setMemory } from "./env.js";

// The wasm module is loaded dynamically (rather than a static import) so it can
// be RE-INSTANTIATED after a wasm trap. The faithful translation emulates Lua's
// C++ exceptions with Rust `panic_any` + `catch_unwind` (parser ParseError, VM
// runtime errors / `error()` / failed `pcall`). On wasm32-unknown-unknown the
// stable std only supports `panic = "abort"`, so those recoverable panics abort
// the instance instead of unwinding. We keep the standard stable build and make
// the playground resilient: a trapped call is caught in JS and the engine is
// transparently reloaded, so the playground never gets stuck. (A successful run
// or type-check returns normally and needs no reload.)
let engine = null; // { run, check }
let engineGen = 0; // cache-buster so each reload gets a fresh module instance

async function loadEngine() {
  engineGen += 1;
  const mod = await import(`./pkg/luaur_web.js?gen=${engineGen}`);
  const wasm = await mod.default();
  setMemory(wasm.memory);
  engine = { run: mod.run, check: mod.check };
}

// CodeMirror 6 is vendored as a single self-contained ESM bundle
// (codemirror.bundle.js). Loading the CM packages individually from a CDN
// pulls in multiple copies of @codemirror/state and breaks CodeMirror's
// internal `instanceof` checks ("Unrecognized extension value"); one local
// bundle guarantees a single shared @codemirror/state instance and works
// offline with no CDN version-matrix fragility.
import {
  EditorView,
  EditorState,
  basicSetup,
  keymap,
  StreamLanguage,
  lua,
  oneDark,
} from "./codemirror.bundle.js";

// ─────────────────────────── examples (inlined) ───────────────────────────
// Inlined so the page works as a pure static file (no fetch / CORS needed),
// even when opened over file://. These are the same scripts shipped in
// website/examples/*.luau.
const EXAMPLES = {
  hello: {
    label: "Hello, world (basics)",
    source: `-- hello.luau
-- The classic first program: print a greeting, then show a few basics.

-- \`print\` accepts multiple arguments, separated by tabs in the output.
print("Hello, world!")

-- Numbers are double-precision; arithmetic works as expected.
local a = 7
local b = 5
print("sum:", a + b)          -- 12
print("product:", a * b)      -- 35
print("power:", a ^ 2)        -- 49 (^ is exponentiation)
print("remainder:", a % b)    -- 2  (% is modulo)

-- Strings are joined with the \`..\` concatenation operator.
local name = "Luau"
print("Welcome to " .. name .. "!")

-- Numbers are coerced to strings inside concatenation.
print("a + b = " .. (a + b))

-- \`print\` with several values of mixed types on one line.
print("mixed:", 42, true, "text", nil)
`,
  },
  fibonacci: {
    label: "Fibonacci (recursion)",
    source: `-- fibonacci.luau
-- Two ways to compute Fibonacci numbers: recursion and iteration.

-- Recursive definition: fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2).
local function fibRecursive(n)
\tif n < 2 then
\t\treturn n
\tend
\treturn fibRecursive(n - 1) + fibRecursive(n - 2)
end

-- Iterative definition: keep the last two values and step forward.
local function fibIterative(n)
\tlocal prev, curr = 0, 1
\tfor _ = 1, n do
\t\tprev, curr = curr, prev + curr
\tend
\treturn prev
end

print("Recursive:")
for i = 0, 9 do
\tprint(i, fibRecursive(i))
end

print("Iterative:")
for i = 0, 9 do
\tprint(i, fibIterative(i))
end
`,
  },
  tables: {
    label: "Tables (arrays + dictionaries)",
    source: `-- tables.luau
-- Tables are Luau's single data structure: they act as arrays AND dictionaries.

-- An array-like table: consecutive integer keys starting at 1.
local fruits = { "apple", "banana", "cherry" }

-- \`#t\` gives the length (number of array entries).
print("number of fruits:", #fruits)

-- \`ipairs\` walks array entries in order (1, 2, 3, ...).
print("fruits in order:")
for index, fruit in ipairs(fruits) do
\tprint(index, fruit)
end

-- \`table.insert\` appends to the end of the array part.
table.insert(fruits, "date")
print("after insert, length:", #fruits)

-- A dictionary-like table: arbitrary string keys mapping to values.
local ages = { alice = 30, bob = 25, carol = 41 }

-- \`pairs\` walks every key/value pair.
print("ages:")
for name, age in pairs(ages) do
\tprint(name, age)
end

-- Tables can mix and nest freely.
local person = { name = "Dave", hobbies = { "chess", "cycling" } }
print(person.name .. " enjoys " .. person.hobbies[1] .. " and " .. person.hobbies[2])
`,
  },
  metatables: {
    label: "Metatables (OOP + operators)",
    source: `-- metatables.luau
-- Object-oriented programming in Luau is built on metatables.
-- Here we make a small Vector2 "class".

local Vector2 = {}
-- \`__index = Vector2\` means: missing fields on an instance fall back to Vector2.
Vector2.__index = Vector2

-- Constructor.
function Vector2.new(x, y)
\tlocal self = setmetatable({}, Vector2)
\tself.x = x
\tself.y = y
\treturn self
end

-- A method. \`self\` is the instance (via the \`:\` call syntax).
function Vector2:magnitude()
\treturn (self.x * self.x + self.y * self.y) ^ 0.5
end

-- Operator overloading: \`__add\` is called for the \`+\` operator.
function Vector2.__add(a, b)
\treturn Vector2.new(a.x + b.x, a.y + b.y)
end

function Vector2:toString()
\treturn "(" .. self.x .. ", " .. self.y .. ")"
end

local v1 = Vector2.new(3, 4)
local v2 = Vector2.new(1, 2)

print("v1:", v1:toString())
print("v2:", v2:toString())
print("v1 magnitude:", v1:magnitude())   -- 5

local sum = v1 + v2                        -- dispatches to __add
print("v1 + v2:", sum:toString())          -- (4, 6)
`,
  },
  strings: {
    label: "Strings (string library)",
    source: `-- strings.luau
-- A tour of the string library.

local s = "Hello, Luau"

print("length:", #s)                         -- byte length
print("upper:", string.upper(s))
print("lower:", string.lower(s))
print("sub(1, 5):", string.sub(s, 1, 5))     -- "Hello"
print("sub(8):", string.sub(s, 8))            -- "Luau"
print("rep:", string.rep("ab", 3))            -- "ababab"

-- string.format works like C printf.
print(string.format("name=%s pi=%.2f count=%d", "Luau", 3.14159, 42))

-- string.find returns the start and end indices of a match (or nil).
local start, finish = string.find(s, "Luau")
print("found 'Luau' at:", start, finish)      -- 8  11

-- gsub does a global substitution and returns the result plus a count.
local replaced, count = s:gsub("l", "L")
print("gsub result:", replaced)
print("gsub count:", count)
`,
  },
  coroutines: {
    label: "Coroutines (generators)",
    source: `-- coroutines.luau
-- Coroutines are cooperative, resumable functions: they pause with
-- \`coroutine.yield\` and continue with \`coroutine.resume\`.

local function producer()
\tcoroutine.yield("first")
\tcoroutine.yield("second")
\tcoroutine.yield("third")
\treturn "done"
end

local routine = coroutine.create(producer)

-- Each resume runs until the next yield (or return).
print("resume 1:", coroutine.resume(routine))   -- true  first
print("resume 2:", coroutine.resume(routine))   -- true  second
print("resume 3:", coroutine.resume(routine))   -- true  third
print("resume 4:", coroutine.resume(routine))   -- true  done
print("status:", coroutine.status(routine))      -- dead

-- coroutine.wrap turns a coroutine into a plain function.
local function squaresUpTo(n)
\treturn coroutine.wrap(function()
\t\tfor i = 1, n do
\t\t\tcoroutine.yield(i * i)
\t\tend
\tend)
end

print("squares:")
local nextSquare = squaresUpTo(5)
for _ = 1, 5 do
\tprint(nextSquare())
end
`,
  },
  typed: {
    label: "Typed (--!strict, type-checks clean)",
    source: `--!strict
-- typed.luau
-- Luau is gradually typed. With \`--!strict\`, the type checker verifies
-- annotations. This file is correct and type-checks cleanly.

-- A type alias names a shape so it can be reused.
type Point = { x: number, y: number }

local greeting: string = "Typed Luau"
local count: number = 3

-- A function with typed parameters and a typed return value.
local function distance(a: Point, b: Point): number
\tlocal dx: number = a.x - b.x
\tlocal dy: number = a.y - b.y
\treturn (dx * dx + dy * dy) ^ 0.5
end

local origin: Point = { x = 0, y = 0 }
local target: Point = { x = 3, y = 4 }

print(greeting)
print("count:", count)
print("distance:", distance(origin, target))   -- 5

-- An array typed as a list of numbers.
local scores: { number } = { 10, 20, 30 }
local total: number = 0
for _, score in ipairs(scores) do
\ttotal += score
end
print("total score:", total)   -- 60
`,
  },
  type_error: {
    label: "Type error (run Type-check!)",
    source: `--!strict
-- type_error.luau
-- This file is DELIBERATELY wrong. The syntax is valid (it parses), but the
-- types do not line up. Press "Type-check" to see the analyzer catch it.

-- A function that expects a number and returns a number.
local function double(n: number): number
\treturn n * 2
end

-- ERROR: \`count\` is declared as a number but assigned a string literal.
local count: number = "not a number"

-- ERROR: \`double\` expects a number argument, but we pass a string.
print(double("ten"))

print(count)
`,
  },
};

const DEFAULT_EXAMPLE = "hello";

// ─────────────────────────── DOM refs ───────────────────────────
const $ = (id) => document.getElementById(id);
const elOutput = $("output");
const elStatus = $("status");
const elRun = $("btn-run");
const elCheck = $("btn-check");
const elClear = $("btn-clear");
const elSelect = $("example-select");

let editor = null;

// ─────────────────────────── editor ───────────────────────────
function makeEditor(initialDoc) {
  const runShortcut = keymap.of([
    {
      key: "Mod-Enter",
      run: () => {
        doRun();
        return true;
      },
    },
  ]);

  const state = EditorState.create({
    doc: initialDoc,
    extensions: [
      basicSetup,
      StreamLanguage.define(lua),
      oneDark,
      runShortcut,
      EditorView.theme({
        "&": { height: "100%", backgroundColor: "transparent" },
        ".cm-scroller": { overflow: "auto" },
        ".cm-gutters": { backgroundColor: "transparent", border: "none" },
      }),
    ],
  });

  return new EditorView({ state, parent: $("editor") });
}

function getSource() {
  return editor ? editor.state.doc.toString() : "";
}

function setSource(text) {
  if (!editor) return;
  editor.dispatch({
    changes: { from: 0, to: editor.state.doc.length, insert: text },
  });
}

// ─────────────────────────── output helpers ───────────────────────────
function setStatus(text, kind) {
  elStatus.textContent = text;
  elStatus.className = "status status-" + kind;
}

function escapeHtml(s) {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function writeOutput(text, kind) {
  const cls = kind ? ` class="${kind}"` : "";
  elOutput.innerHTML = `<span${cls}>${escapeHtml(text)}</span>`;
}

// A wasm trap (from an un-unwindable panic = a Lua error: parse error, runtime
// error, error(), failed pcall) leaves the instance poisoned. Detect it,
// explain it, and transparently reload a fresh engine so the next run works.
function isTrap(e) {
  return e instanceof WebAssembly.RuntimeError || /unreachable|table index|out of bounds/i.test(String(e && e.message));
}

async function recoverFromTrap() {
  try {
    await loadEngine();
    return true;
  } catch (_) {
    return false;
  }
}

// ─────────────────────────── actions ───────────────────────────
async function doRun() {
  if (!engine) return;
  const src = getSource();
  setRunning(true);
  setStatus("running…", "running");
  await frame();
  let result;
  try {
    result = engine.run(src);
  } catch (e) {
    if (isTrap(e)) {
      writeOutput(
        "The script raised a Lua error (a runtime error, error(), a failed\n" +
          "assert/pcall, or a syntax error). The engine has been reloaded — edit\n" +
          "the script and run again.",
        "out-err"
      );
      setStatus("lua error", "error");
      await recoverFromTrap();
    } else {
      writeOutput("internal error: " + msg(e), "out-err");
      setStatus("error", "error");
    }
    setRunning(false);
    return;
  }
  const looksError = /\b(error|Error)\b|stack backtrace|attempt to/.test(result);
  if (result.trim() === "") {
    writeOutput("(no output — the script produced no print results)", "out-meta");
    setStatus("ran ok", "ready");
  } else if (looksError) {
    writeOutput(result, "out-err");
    setStatus("runtime error", "error");
  } else {
    writeOutput(result, "out-ok");
    setStatus("ran ok", "ready");
  }
  setRunning(false);
}

async function doCheck() {
  if (!engine) return;
  const src = getSource();
  setRunning(true);
  setStatus("checking…", "running");
  await frame();
  let result;
  try {
    result = engine.check(src);
  } catch (e) {
    if (isTrap(e)) {
      writeOutput(
        "The analyzer hit an unrecoverable error on this input (it may use a\n" +
          "code path that relies on stack unwinding, unavailable on this wasm\n" +
          "target). The engine has been reloaded.",
        "out-err"
      );
      setStatus("error", "error");
      await recoverFromTrap();
    } else {
      writeOutput("internal error: " + msg(e), "out-err");
      setStatus("error", "error");
    }
    setRunning(false);
    return;
  }
  if (result.trim() === "No errors." || result.trim() === "") {
    writeOutput("No type errors.\nThe analyzer accepts this program.", "out-ok");
    setStatus("type-checked", "ready");
  } else {
    writeOutput("Type-checker diagnostics:\n\n" + result, "out-err");
    setStatus("type errors", "error");
  }
  setRunning(false);
}

function doClear() {
  writeOutput("", "");
  if (engine) setStatus("ready", "ready");
}

// helpers
const msg = (e) => (e && e.message ? e.message : String(e));
const frame = () => new Promise((r) => requestAnimationFrame(() => r()));
function setRunning(on) {
  elRun.disabled = on || !engine;
  elCheck.disabled = on || !engine;
}

// ─────────────────────────── boot ───────────────────────────
function populateExamples() {
  for (const [key, ex] of Object.entries(EXAMPLES)) {
    const opt = document.createElement("option");
    opt.value = key;
    opt.textContent = ex.label;
    elSelect.appendChild(opt);
  }
  elSelect.value = DEFAULT_EXAMPLE;
  elSelect.addEventListener("change", () => {
    const ex = EXAMPLES[elSelect.value];
    if (ex) {
      setSource(ex.source);
      doClear();
    }
  });
}

async function boot() {
  populateExamples();
  editor = makeEditor(EXAMPLES[DEFAULT_EXAMPLE].source);

  elRun.addEventListener("click", doRun);
  elCheck.addEventListener("click", doCheck);
  elClear.addEventListener("click", doClear);

  try {
    // loadEngine() instantiates the wasm and wires env.js to its linear memory.
    await loadEngine();
    elRun.disabled = false;
    elCheck.disabled = false;
    setStatus("ready", "ready");
    writeOutput(
      'Engine ready. Press "Run" to execute, or "Type-check" to analyze.\n' +
        "Try the examples in the dropdown above — including the deliberate type error.",
      "out-meta"
    );
  } catch (e) {
    setStatus("wasm failed", "error");
    writeOutput(
      "Failed to load the WebAssembly engine.\n" + (e && e.message ? e.message : e),
      "out-err"
    );
    console.error(e);
  }
}

boot();
