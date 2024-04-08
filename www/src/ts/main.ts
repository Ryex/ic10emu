import "@popperjs/core";
import "../scss/styles.scss";
import { Dropdown, Modal } from "bootstrap";
import "./app";

// A dependency graph that contains any wasm must all be imported
// asynchronously. This `main.js` file does the single async import, so
// that no one else needs to worry about it again.
// import("./index")
//   .catch(e => console.error("Error importing `index.ts`:", e));
