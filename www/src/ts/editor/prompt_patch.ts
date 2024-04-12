import { prompt as ace_prompt } from "ace-builds/src-noconflict/ext-prompt"

console.log(ace_prompt);

function prompt(editor: { cmdLine: { setTheme: (arg0: string) => void; }; }, message: any, options: any, callback: any) {
  ace_prompt(editor, message, options, callback);
  if (editor.cmdLine) {
    editor.cmdLine.setTheme("ace/theme/one_dark");
  }
}

export { prompt };
