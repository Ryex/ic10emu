
import type { Editor } from "ace-code";
import { prompt as ace_prompt } from "ace-code/src/ext/prompt";

function prompt(editor: Editor, message: any, options: any, callback: any) {
  ace_prompt(editor, message, options, callback);
  if (editor.cmdLine) {
    editor.cmdLine.setTheme("ace/theme/one_dark");
  }
}

export { prompt };
