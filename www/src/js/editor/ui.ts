import { IC10Editor } from ".";
import * as ace from "ace-builds";
import { Offcanvas } from 'bootstrap';

class IC10EditorUI {
  ic10editor: IC10Editor;

  constructor(ic10editor: IC10Editor) {

    const that = this;

    that.ic10editor = ic10editor;

    // that.ic10editor.editor.commands.addCommand({
    //   name: "showSettingsMenu",
    //   description: "Show settings menu",
    //   bindKey: { win: "Ctrl-,", mac: "Command-," },
    //   exec: (_editor: ace.Ace.Editor) => {
    //     const offCanvas = new Offcanvas(document.getElementById("editorSettings"));
    //     offCanvas.toggle();
    //   }
    // } as any);

    ace.config.loadModule("ace/ext/keyboard_menu", function (module) {
      console.log("keybinding_menu loaded");
      module.init(that.ic10editor.editor);
    });

    that.ic10editor.loadEditorSettings();
    that.displayEditorSettings();
    that.updateEditorSettings();
    that.reCalcEditorSize();
    window.addEventListener('resize', (e) => { that.reCalcEditorSize(); });

    document.getElementsByName("editorKeybindRadio").forEach((el) => {
      el.addEventListener('change', (e) => {
        that.ic10editor.settings.keyboard = (e.target as any).value;
        that.ic10editor.saveEditorSettings();
        that.updateEditorSettings();
      });
    });

    document.getElementsByName("editorCursorRadio").forEach((el) => {
      el.addEventListener('change', (e) => {
        that.ic10editor.settings.cursor = (e.target as any).value;
        that.ic10editor.saveEditorSettings();
        that.updateEditorSettings();
      });
    });
    document.getElementById("editorSettingsFontSize").addEventListener('change', (e) => {
      window.App.editorSettings.fontSize = parseInt((e.target as any).value);
      that.ic10editor.saveEditorSettings();
      that.updateEditorSettings();
    });
    document.getElementById("editorSettingsRelativeLineNumbers").addEventListener('change', (e) => {
      window.App.editorSettings.relativeLineNumbers = (e.target as any).checked;
      that.ic10editor.saveEditorSettings();
      that.updateEditorSettings();
    });

    console.log(that.ic10editor.editor.getOption('keyboardHandler'));

    that.ic10editor.editor.setTheme("ace/theme/one_dark");

    that.ic10editor.editor.setAutoScrollEditorIntoView(true);

  }

  updateEditorSettings() {
    const settings = this.ic10editor.settings;
    const editor = this.ic10editor.editor;
    if (settings.keyboard === 'ace') {
      editor.setOption('keyboardHandler', null);
    } else {
      editor.setOption('keyboardHandler', `ace/keyboard/${settings.keyboard}`);
    }
    editor.setOption('cursorStyle', settings.cursor as any);
    editor.setOption('fontSize', settings.fontSize);
    editor.setOption('relativeLineNumbers', settings.relativeLineNumbers);
  }

  displayEditorSettings() {
    const settings = this.ic10editor.settings;
    document.getElementsByName("editorKeybindRadio").forEach((el: any) => {
      el.checked = el.value === settings.keyboard;
    });
    document.getElementsByName("editorCursorRadio").forEach((el: any) => {
      el.checked = el.value === settings.cursor;
    });
    (document.getElementById("editorSettingsFontSize") as any).value = settings.fontSize;
    (document.getElementById("editorSettingsRelativeLineNumbers") as any).checked = settings.relativeLineNumbers;
  }

  reCalcEditorSize() {
    const editor = this.ic10editor.editor;
    const navBar = document.getElementById("navBar");
    const statusBarContainer = document.getElementById("statusBarContainer");

    const correction = navBar.offsetHeight + statusBarContainer.offsetHeight;
    const editorContainer = document.getElementById("editor");
    editorContainer.style.height = `calc( 100vh - ${correction}px - 0.5rem)`;
    editor.resize(true);
  }
}




export { IC10EditorUI };
