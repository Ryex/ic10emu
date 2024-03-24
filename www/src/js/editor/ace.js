import * as ace from "ace-code"

// make sure Ace can load through webpack
// trimmed down version of https://github.com/ajaxorg/ace-builds/blob/master/esm-resolver.js but for ace-code
ace.config.setModuleLoader("ace/theme/one_dark", () => import("ace-code/src/theme/one_dark"));
ace.config.setModuleLoader("ace/theme/textmate", () => import("ace-code/src/theme/textmate"));

ace.config.setModuleLoader('ace/ext/beautify', () => import('ace-code/src/ext/beautify.js'));
ace.config.setModuleLoader('ace/ext/code_lens', () => import('ace-code/src/ext/code_lens.js'));
ace.config.setModuleLoader('ace/ext/command_bar', () => import('ace-code/src/ext/command_bar.js'));
ace.config.setModuleLoader('ace/ext/elastic_tabstops_lite', () => import('ace-code/src/ext/elastic_tabstops_lite.js'));
ace.config.setModuleLoader('ace/ext/emmet', () => import('ace-code/src/ext/emmet.js'));
ace.config.setModuleLoader('ace/ext/error_marker', () => import('ace-code/src/ext/error_marker.js'));
ace.config.setModuleLoader('ace/ext/hardwrap', () => import('ace-code/src/ext/hardwrap.js'));
ace.config.setModuleLoader('ace/ext/inline_autocomplete', () => import('ace-code/src/ext/inline_autocomplete.js'));
ace.config.setModuleLoader('ace/ext/keyboard_menu', () => import('ace-code/src/ext/keybinding_menu.js'));
ace.config.setModuleLoader('ace/ext/language_tools', () => import('ace-code/src/ext/language_tools.js'));
ace.config.setModuleLoader('ace/ext/linking', () => import('ace-code/src/ext/linking.js'));
ace.config.setModuleLoader('ace/ext/modelist', () => import('ace-code/src/ext/modelist.js'));
ace.config.setModuleLoader('ace/ext/options', () => import('ace-code/src/ext/options.js'));
// ace.config.setModuleLoader('ace/ext/prompt', () => import('ace-code/src/ext/prompt.js'));
ace.config.setModuleLoader('ace/ext/prompt', () => import('./prompt_patch'));
ace.config.setModuleLoader('ace/ext/rtl', () => import('ace-code/src/ext/rtl.js'));
ace.config.setModuleLoader('ace/ext/searchbox', () => import('ace-code/src/ext/searchbox.js'));
// ace.config.setModuleLoader('ace/ext/settings_menu', () => import('ace-code/src/ext/settings_menu.js'));
ace.config.setModuleLoader('ace/ext/simple_tokenizer', () => import('ace-code/src/ext/simple_tokenizer.js'));
ace.config.setModuleLoader('ace/ext/spellcheck', () => import('ace-code/src/ext/spellcheck.js'));
ace.config.setModuleLoader('ace/ext/split', () => import('ace-code/src/ext/split.js'));
ace.config.setModuleLoader('ace/ext/static_highlight', () => import('ace-code/src/ext/static_highlight.js'));
ace.config.setModuleLoader('ace/ext/statusbar', () => import('ace-code/src/ext/statusbar.js'));
ace.config.setModuleLoader('ace/ext/textarea', () => import('ace-code/src/ext/textarea.js'));
ace.config.setModuleLoader('ace/ext/themelist', () => import('ace-code/src/ext/themelist.js'));
ace.config.setModuleLoader('ace/ext/whitespace', () => import('ace-code/src/ext/whitespace.js'));
ace.config.setModuleLoader('ace/keyboard/emacs', () => import('ace-code/src/keyboard/emacs.js'));
ace.config.setModuleLoader('ace/keyboard/sublime', () => import('ace-code/src/keyboard/sublime.js'));
ace.config.setModuleLoader('ace/keyboard/vim', () => import('ace-code/src/keyboard/vim.js'));
ace.config.setModuleLoader('ace/keyboard/vscode', () => import('ace-code/src/keyboard/vscode.js'));

console.log("ace module loaders patched");

export { ace };
