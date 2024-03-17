"use strict";

var oop = require("ace-code/src/lib/oop");
var TextMode = require("ace-code/src/mode/text").Mode;
var FoldMode = require("ace-code/src/mode/folding/cstyle").FoldMode;
var IC10HighlightRules = require("./ic10_highlight_rules.js").IC10HighlightRules;

var Mode = function() {
    this.HighlightRules = IC10HighlightRules;
    this.foldingRules = new FoldMode();
}

oop.inherits(Mode, TextMode);

(function() {
    // configure comment start/end characters
    this.lineCommentStart = ["#"];
    this.$id = "ace/mode/ic10";
    
}).call(Mode.prototype);

exports.Mode = Mode;
