import { ace } from "./ace";
import { Offcanvas } from 'bootstrap';

function IC10EditorUI(ic10editor) {

  const self = this;

  self.ic10editor = ic10editor;

  self.ic10editor.editor.commands.addCommand({
    name: "showSettingsMenu",
    description: "Show settings menu",
    bindKey: { win: "Ctrl-,", mac: "Command-," },
    exec: (_editor) => {
      const offCanvas = new Offcanvas(document.getElementById("editorSettings"));
      offCanvas.toggle();
    }
  });

  ace.config.loadModule("ace/ext/keyboard_menu", function(module) {
    console.log("keybinding_menu loaded");
    module.init(self.ic10editor.editor);
  })

  self.ic10editor.loadEditorSettings();
  self.displayEditorSettings();
  self.updateEditorSettings();
  self.reCalcEditorSize();
  window.addEventListener('resize', (e) => { self.reCalcEditorSize() });

  document.getElementsByName("editorKeybindRadio").forEach((el) => {
    el.addEventListener('change', (e) => {
      self.ic10editor.settings.keyboard = e.target.value;
      self.ic10editor.saveEditorSettings();
      self.updateEditorSettings();
    })
  });

  document.getElementsByName("editorCursorRadio").forEach((el) => {
    el.addEventListener('change', (e) => {
      self.ic10editor.settings.cursor = e.target.value;
      self.ic10editor.saveEditorSettings();
      self.updateEditorSettings();
    })
  });
  document.getElementById("editorSettingsFontSize").addEventListener('change', (e) => {
    window.App.editorSettings.fontSize = e.target.value;
    self.ic10editor.saveEditorSettings();
    self.updateEditorSettings();
  });
  document.getElementById("editorSettingsRelativeLineNumbers").addEventListener('change', (e) => {
    window.App.editorSettings.relativeLineNumbers = e.target.checked;
    self.ic10editor.saveEditorSettings();
    self.updateEditorSettings();
  })

  console.log(self.ic10editor.editor.getOption('keyboardHandler'));

  self.ic10editor.editor.setTheme("ace/theme/one_dark");
  ace.config.loadModule("ace/ext/statusbar", function(module) {
    const statusBar = new module.StatusBar(self.ic10editor.editor, document.getElementById("statusBar"));
    statusBar.updateStatus(self.ic10editor.editor);
  })

  self.ic10editor.editor.setAutoScrollEditorIntoView(true);
}

IC10EditorUI.prototype.updateEditorSettings = function() {
  const settings = this.ic10editor.settings;
  const editor = this.ic10editor.editor;
  if (settings.keyboard === 'ace') {
    editor.setOption('keyboardHandler', null);
  } else {
    editor.setOption('keyboardHandler', `ace/keyboard/${settings.keyboard}`);
  }
  editor.setOption('cursorStyle', settings.cursor);
  editor.setOption('fontSize', `${settings.fontSize}px`);
  editor.setOption('relativeLineNumbers', settings.relativeLineNumbers);
}

IC10EditorUI.prototype.displayEditorSettings = function() {
  const settings = this.ic10editor.settings;
  document.getElementsByName("editorKeybindRadio").forEach((el) => {
    el.checked = el.value === settings.keyboard;
  });
  document.getElementsByName("editorCursorRadio").forEach((el) => {
    el.checked = el.value === settings.cursor;
  });
  document.getElementById("editorSettingsFontSize").value = settings.fontSize;
  document.getElementById("editorSettingsRelativeLineNumbers").checked = settings.relativeLineNumbers;
}

IC10EditorUI.prototype.reCalcEditorSize = function() {
  const editor = this.ic10editor.editor;
  const navBar = document.getElementById("navBar");
  const statusBarContainer = document.getElementById("statusBarContainer");

  const correction = navBar.offsetHeight + statusBarContainer.offsetHeight;
  const editorContainer = document.getElementById("editor");
  editorContainer.style.height = `calc( 100vh - ${correction}px - 0.5rem)`;
  editor.resize(true);
}

export { IC10EditorUI };
