import ace from "ace-code";
import { Mode as TextMode } from "ace-code/src/mode/text";
import { TextHighlightRules } from "ace-code/src/mode/text_highlight_rules";
import * as hl_parts from "ic10_hl_parts";

const MODE_NAME = "ace/mode/ic10";

class IC10HighlightRules extends TextHighlightRules {
  constructor() {
    super();

    const opsMapper = this.createKeywordMapper(
      { keyword: hl_parts.ops },
      "text",
      false,
      " ",
    );

    // https://regex101.com/r/XWgqpN/1  << potentially useful regex for labeling jumps
    // regexp must not have capturing parentheses. Use (?:) instead.
    // regexps are ordered -> the first match is used

    this.$rules = {
      start: [
        {
          token: "comment.line.number-sign",
          regex: /#.*$/,
        },
        {
          token: ["text", "keyword.control", "text", "entity.name.tag"],
          regex: /^(\s*)(j(?:al)?)(\s+)([a-zA-Z_.][a-zA-Z0-9_.]*)/,
          comment: "absolute jumps | branches with a label",
        },
        {
          token: "entity.name.tag",
          regex: /^\s*[a-zA-Z_.][a-zA-Z0-9_.]*\b:/,
          comment: "Labels at line start: begin_repeat: add ...",
        },
        {
          token: opsMapper,
          regex: /^\s*(\w+)$/,
          stateName: "operation",
          next: "start",
        },
        {
          token: opsMapper,
          regex: /^\s*(\w+)\b/,
          stateName: "operation",
          next: "operationContext",
        },
      ],
      operationContext: [
        {
          token: "support.function",
          regex: /HASH|STR/,
          stateName: "preproc",
          next: [
            {
              token: "paren.lparen",
              regex: /\(/,
              stateName: "preprocp",
              next: [
                {
                  token: "string.start",
                  regex: /"/,
                  stateName: "preprocpq",
                  next: [
                    { token: "text", regex: /$/, next: "start" },

                    {
                      token: "string.end",
                      regex: /"/,
                      next: [
                        { token: "text", regex: /$/, next: "start" },
                        {
                          token: "paren.rparen",
                          regex: /\)/,
                          next: "operationContext",
                        },
                        { defaultToken: "text" },
                      ],
                    },
                    { defaultToken: "string" },
                  ],
                },

                { token: "text", regex: /$/, next: "start" },
                { defaultToken: "text" },
              ],
            },
            { token: "text", regex: /$/, next: "start" },
            { defaultToken: "text" },
          ],
        },
        {
          token: "comment.line.number-sign",
          regex: /#.*$/,
          next: "start",
        },
        {
          token: "constant.numeric",
          regex: /\b%[01_]+/,
          comment: "Numbers like %1000",
        },
        {
          token: "constant.numeric",
          regex: /\b\$[a-fA-F0-9_]+/,
          comment: "Numbers like $3F",
        },
        {
          token: "constant.numeric",
          regex: /\b\-?[0-9]+(?:\.[0-9]+)?/,
          comment: "Numbers like 12, -3.0, 55",
        },
        {
          token: "variable.parameter",
          regex: /\b(?:sp|r(?:a|r*)(?:[0-9]|1[0-7]))/,
        },
        {
          token: "storage.modifier",
          regex: /\b(?:d(?:b|[0-5]|r*(?:[0-9]|1[0-7]))(?::[0-9]+))/,
          comment: "Devices",
        },
        {
          token: "storage.type",
          regex: /\b(?:d(?:b|[0-5]|r*(?:[0-9]|1[0-7])))/,
          comment: "Device Networks",
        },
        {
          token: "invalid.deprecated",
          regex: "\\b(?:" + hl_parts.deprecated.split(" ").join("|") + ")\\b",
        },
        {
          token: "support.type",
          regex:
            "\\b(?:" +
            [
              hl_parts.LogicType,
              hl_parts.LogicSlotType,
              hl_parts.LogicBatchMethod,
              hl_parts.LogicReagentMode,
            ]
              .join(" ")
              .split(" ")
              .join("|") +
            ")\\b",
        },
        {
          token: "variable.language",
          regex:
            "\\b(?:" +
            hl_parts.enums.replace(".", "\\.").split(" ").join("|") +
            ")\\b",
        },
        {
          token: "constant.language",
          regex: "\\b(?:" + hl_parts.constants.split(" ").join("|") + ")\\b",
        },
        {
          token: "entity.name",
          regex: /[a-zA-Z_.][a-zA-Z0-9_.]*/,
        },
        { token: "text", regex: /\s/ },
        { token: "text", regex: /\n|$/, next: "start" },
      ],
    };

    this.normalizeRules();
  }
}

class IC10Mode extends TextMode {
  $id: string;
  constructor() {
    super();
    this.HighlightRules = IC10HighlightRules;

    this.lineCommentStart = "#";
    this.$id = MODE_NAME;
  }
}

export { IC10Mode as Mode };
