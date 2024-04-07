import { css } from "lit";
export const editorStyles = css`
  :host {
    display: block;
    width: 100%;
    height: 100%;
  }
  #editor {
    // border: 1px solid;
    // border-radius: 4px;
    // @apply --ace-widget-editor;
  }
  #editorStatusbar {
    z-index: 9 !important;
    position: absolute !important;
    right: 4px;
    bottom: 4px;
  }
  .ace_status-indicator {
    background-color: #777;
    color: white;
    text-align: center;
    border: none;
    border-radius: 7px;
    padding-right: 3px;
    padding-left: 3px;
    padding-bottom: 1px;
    font-size: small;
    opacity: 0.9;
  }
  .ace_marker-layer .green {
    // background-color: ;
    // color: ;
    position: absolute;
  }
  .ace_marker-layer .darkGrey {
    // background-color: ;
    // color: ;
    position: absolute;
  }
  .ace_marker-layer .red {
    // background-color: ;
    // color: ;
    position: absolute;
  }
  .ace_marker-layer .blue {
    // background-color: ;
    // color: ;
    position: absolute;
  }
  .ace_marker-layer .orange {
    background-color: #ff9900;
    color: #555;
    position: absolute;
  }
  .ace_placeholder {
    color: #808080 !important;
    // font-family: "" !important;
    transform: scale(1) !important;
    opacity: 1 !important;
    font-style: italic !important;
  }
  /* ------------------------------------------------------------------------------------------
  * Editor Search Form
  * --------------------------------------------------------------------------------------- */
  .ace_search {
    background-color: #2b3035;
    color: #dee2e6;
    border: 1px solid #495057;
    border-top: 0 none;
    overflow: hidden;
    margin: 0;
    padding: 4px 6px 0 4px;
    position: absolute;
    top: 0;
    z-index: 99;
    white-space: normal;
  }

  .ace_search.left {
    border-left: 0 none;
    border-radius: 0px 0px 5px 0px;
    left: 0;
  }

  .ace_search.right {
    border-radius: 0px 0px 0px 5px;
    border-right: 0 none;
    right: 0;
  }

  .ace_search_form,
  .ace_replace_form {
    margin: 0 20px 4px 0;
    overflow: hidden;
    line-height: 1.9;
  }

  .ace_replace_form {
    margin-right: 0;
  }

  .ace_search_form.ace_nomatch {
    outline: 1px solid red;
  }

  .ace_search_field {
    border-radius: 3px 0 0 3px;
    background-color: #343a40;
    color: #dee2e6;
    border: 1px solid #41464b;
    border-right: 0 none;
    outline: 0;
    padding: 0;
    font-size: inherit;
    margin: 0;
    line-height: inherit;
    padding: 0 6px;
    min-width: 17em;
    vertical-align: top;
    min-height: 1.8em;
    box-sizing: content-box;
  }

  .ace_searchbtn {
    border: 1px solid #6c757d;
    line-height: inherit;
    display: inline-block;
    padding: 0 6px;
    background: #343a40;
    border-right: 0 none;
    border-left: 1px solid #6c757d;
    cursor: pointer;
    margin: 0;
    position: relative;
    color: #fff;
  }

  .ace_searchbtn:last-child {
    border-radius: 0 3px 3px 0;
    border-right: 1px solid #6c757d;
  }

  .ace_searchbtn:disabled {
    background: none;
    cursor: default;
  }

  .ace_searchbtn:hover {
    background-color: #161719;
  }

  .ace_searchbtn.prev,
  .ace_searchbtn.next {
    padding: 0px 0.7em;
  }

  .ace_searchbtn.prev:after,
  .ace_searchbtn.next:after {
    content: "";
    border: solid 2px #6c757d;
    width: 0.5em;
    height: 0.5em;
    border-width: 2px 0 0 2px;
    display: inline-block;
    transform: rotate(-45deg);
  }

  .ace_searchbtn.next:after {
    border-width: 0 2px 2px 0;
  }

  .ace_searchbtn_close {
    background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAcCAYAAABRVo5BAAAAZ0lEQVR42u2SUQrAMAhDvazn8OjZBilCkYVVxiis8H4CT0VrAJb4WHT3C5xU2a2IQZXJjiQIRMdkEoJ5Q2yMqpfDIo+XY4k6h+YXOyKqTIj5REaxloNAd0xiKmAtsTHqW8sR2W5f7gCu5nWFUpVjZwAAAABJRU5ErkJggg==)
      no-repeat 50% 0;
    border-radius: 50%;
    border: 0 none;
    color: #343a40;
    cursor: pointer;
    font: 16px/16px Arial;
    padding: 0;
    height: 14px;
    width: 14px;
    top: 9px;
    right: 7px;
    position: absolute;
  }

  .ace_searchbtn_close:hover {
    background-color: #656565;
    background-position: 50% 100%;
    color: white;
  }

  .ace_button {
    background-color: #343a40;
    margin-left: 2px;
    cursor: pointer;
    -webkit-user-select: none;
    -moz-user-select: none;
    -o-user-select: none;
    -ms-user-select: none;
    user-select: none;
    overflow: hidden;
    opacity: 0.7;
    border: 1px solid #6c757d;
    padding: 1px;
    box-sizing: border-box !important;
    color: #fff;
  }

  .ace_button:hover {
    background-color: #161719;
    opacity: 1;
  }

  .ace_button:active {
    background-color: #6c757d;
  }

  .ace_button.checked {
    background-color: #6c757d;
    border-color: #6c757d;
    opacity: 1;
  }

  .ace_search_options {
    margin-bottom: 3px;
    text-align: right;
    -webkit-user-select: none;
    -moz-user-select: none;
    -o-user-select: none;
    -ms-user-select: none;
    user-select: none;
    clear: both;
  }

  .ace_search_counter {
    float: left;
    font-family: arial;
    padding: 0 8px;
  }

  /* ----------------
  *  Ace Tooltips
  *  --------------- */
  code {
    // color: #e685b5
    color: #c678dd;
  }

  .ace_tooltip code {
    font-style: italic;
    font-size: 12px;
  }
  .ace_tooltip {
    background: #282c34;
    color: #c1c1c1;
    border: 1px #484747 solid;
    box-shadow: 2px 3px 5px rgba(0, 0, 0, 0.51);
  }

  .ace_tooltip.ace_dark {
    background: #282c34;
    color: #c1c1c1;
    border: 1px #484747 solid;
    box-shadow: 2px 3px 5px rgba(0, 0, 0, 0.51);
  }

  /* ----------------
  *  Ace tooltip
  *  --------------- */

  .ace_dark.ace_editor.ace_autocomplete .ace_completion-highlight {
    color: #c678dd;
  }

  .ace_dark.ace_editor.ace_autocomplete .ace_marker-layer .ace_active-line {
    background-color: rgba(76, 87, 103, 0.19);
  }

  .ace_dark.ace_editor.ace_autocomplete .ace_line-hover {
    border: 1px solid rgba(8, 121, 144, 0.5);
    background: rgba(76, 87, 103, 0.19);
  }

  .ace_dark.ace_editor.ace_autocomplete {
    border: 1px #484747 solid;
    box-shadow: 2px 3px 5px rgba(0, 0, 0, 0.51);
    line-height: 1.4;
    background: #282c34;
    color: #c1c1c1;
  }

  .ace_editor.ace_autocomplete {
    width: 300px;
    z-index: 200000;
    border: 1px #484747 solid;
    position: fixed;
    box-shadow: 2px 3px 5px rgba(0, 0, 0, 0.51);
    line-height: 1.4;
    background: #282c34;
    color: #c1c1c1;
  }

  .ace_editor.ace_autocomplete .ace_completion-highlight {
    color: #c678dd;
  }

  .ace_editor.ace_autocomplete .ace_marker-layer .ace_active-line {
    background-color: rgba(76, 87, 103, 0.19);
  }

  .ace_editor.ace_autocomplete .ace_line-hover {
    border: 1px solid rgba(8, 121, 144, 0.5);
    background: rgba(76, 87, 103, 0.19);
  }

  /* ----------------------
  *  Editor Setting dialog
  *  ---------------------- */
  .label-on-left {
    --label-width: 3.75rem;
    --gap-width: 1rem;
  }

  .label-on-left + .label-on-left {
    margin-top: var(--sl-spacing-medium);
  }

  .label-on-left::part(form-control) {
    display: grid;
    grid: auto / var(--label-width) 1fr;
    gap: var(--sl-spacing-3x-small) var(--gap-width);
    align-items: center;
  }

  .label-on-left::part(form-control-label) {
    text-align: right;
  }

  .label-on-left::part(form-control-help-text) {
    grid-column-start: 2;
  }
`;
