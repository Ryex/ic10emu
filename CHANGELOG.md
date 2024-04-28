<!-- insertion marker -->

## [0.2.2] - 2024-04-28

### Summary
This update brings with it functional slots in the UI! Add items to Stackers, Sorters, Vending machines etc. and interact with the relevant data.

** Note: This does not mean that chute networks and internal inventory mechanics are simulated

There was also some work done on the device search UI to vastly improve it's performance.


<small>[Compare with v0.2.1](https://github.com/Ryex/ic10emu/compare/v0.2.1...0.2.2)</small>

### Features

- better slot UI ([c87d3f8](https://github.com/Ryex/ic10emu/commit/c87d3f8bd88a64ad421e5999d7a040de205d4e03) by Rachel Powers).
- much better slot occupant card ([1790715](https://github.com/Ryex/ic10emu/commit/17907151b34bb6efdbd4370cd449e21dcc8eed54) by Rachel Powers).

### Bug Fixes

- device id change UI event chain fixed; changing the Active IC's ID no longer breaks the UI ([4ac823a](https://github.com/Ryex/ic10emu/commit/4ac823a1bc9d3b572de713ac59a5aabd5f0ff599) by Rachel Powers).

### Performance Improvements

- performance improvments ([cfa240c](https://github.com/Ryex/ic10emu/commit/cfa240c5794817ce4221cdac8be2e96e320edf5c) by Rachel Powers).
- vastly improve load speed ([6cc2189](https://github.com/Ryex/ic10emu/commit/6cc21899214296f51e93b70a3f9f67c39ba243d3) by Rachel Powers).
- improve slot UI + device search speedup ([eb4463c](https://github.com/Ryex/ic10emu/commit/eb4463c8ab318e8093e93c1ecaac139cf6dbb74d) by Rachel Powers).

## [v0.2.1]

- prevent borrow panics in VM during batch operations
- fix Maximize batch mode
- fix panic in parsing invalid numbers

<small>[Compare with v0.2.0](https://github.com/Ryex/ic10emu/compare/v0.2.0...v0.2.1)</small>

## [v0.2.0]

### Share VM State!

New in this release is the ability to share the entire VM with you share a link. This means code; connected devices and their state; as well as the state of the stack, registers, and line number of the active IC!

Additionally you can now save and load any number of sessions in your browser. Access this functionality from the main menu.

Also! the project has officially moved to https://ic10emu.dev . Old share links _should_ redirect, but if not simply copy the fragment (the part of the url starting with the `#` symbol)

#### List of changes

- Move build system from Webpack to [Rsbuild](https://rsbuild.dev/) (way faster build times).
- VM now supports exporting and restoring a frozen state.
- Share links updates to use frozen vm state.
- Save and load sessions from the browser's IndexedDB storage.
- project now includes tailwindcss to make frontend dev easier.
- Changelog dialog to notify users of updates.

## [v0.1.0]

### **Initial Release**:

IC10emu is released to the public! edit and share your IC10 scripts!

- view and edit stack and registers
