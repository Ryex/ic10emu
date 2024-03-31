import ace from 'ace-builds';
import { rules as highlightRules } from "./ic10_highlight_rules";

//Support function to create Ace mode
function createAceMode(modeName: string, highlighterObj: ace.Ace.HighlightRulesMap) {
    (ace as any).define(modeName, ["require", "exports", "module"], function (require: any, exports: any, module: any) {
        console.log(require);
        var oop = require("ace/lib/oop");
        var TextMode = require("ace/mode/text").Mode;
        var TextHighlightRules = require("ace/mode/text_highlight_rules").TextHighlightRules;
        var WorkerClient = require("ace/worker/worker_client").WorkerClient;

        var HighlightRules = function () {
            this.$rules = highlighterObj;
            this.normalizeRules();
        };
        oop.inherits(HighlightRules, TextHighlightRules);
        (HighlightRules as any).metaData = {
            fileTypes: [modeName],
            name: modeName,
            scopeName: `source.${modeName}`
        };
        var CustomMode = function () {
            this.HighlightRules = HighlightRules;
        };
        oop.inherits(CustomMode, TextMode);

        (function () {
            //Create worker for live syntax checking
            this.createWorker = function (session: ace.Ace.EditSession) {
                session.on("change", function () {
                    session.clearAnnotations();
                    let annotations: ace.Ace.Annotation[] = [];
                    for (let row = 0; row < session.getLength(); row++) {
                        let tokens = session.getTokens(row);
                        if (!tokens || tokens.length == 0) continue;
                        tokens.forEach(token => {
                            if (token.type === "invalid") annotations.push({ row: row, column: 0, text: "This need to be fixed!", type: "error" });
                        });
                    }
                    session.setAnnotations(annotations);
                });
            }
            this.lineCommentStart = ["#"];
            this.$id = modeName;
        }).call(CustomMode.prototype);
        exports.Mode = CustomMode;
    });
}

createAceMode("ace/mode/ic10", highlightRules);
