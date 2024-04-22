## v0.2.1

 - prevent borrow panics in VM during batch operations
 - fix Maximize batch mode
 - fix panic in parsing invalid numbers

## v0.2.0

### Share VM State!

New in this release is the ability to share the entire VM with you share a link. This means code; connected devices and their state; as well as the state of the stack, registers, and line number of the active IC!

Additionally you can now save and load any number of sessions in your browser. Access this functionality from the main menu.

Also! the project has officially moved to https://ic10emu.dev . Old share links *should* redirect, but if not simply copy the fragment (the part of the url starting with the `#` symbol)

#### List of changes

  - Move build system from Webpack to [Rsbuild](https://rsbuild.dev/) (way faster build times).
  - VM now supports exporting and restoring a frozen state.
  - Share links updates to use frozen vm state.
  - Save and load sessions from the browser's IndexedDB storage.
  - project now includes tailwindcss to make frontend dev easier.
  - Changelog dialog to notify users of updates.

## v0.1.0

### **Initial Release**:

IC10emu is released to the public! edit and share your IC10 scripts!

  - view and edit stack and registers
