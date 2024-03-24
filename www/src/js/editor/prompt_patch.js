const ace_prompt = require('ace-code/src/ext/prompt.js').prompt;
console.log(ace_prompt);

function prompt(editor, message, options, callback) {
  ace_prompt(editor, message, options, callback);
  if (editor.cmdLine) {
    editor.cmdLine.setTheme("ace/theme/one_dark");
  }
}

exports.prompt = prompt;
