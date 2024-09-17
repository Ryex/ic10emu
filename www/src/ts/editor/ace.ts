import ace from "ace-builds";
import "ace-builds/esm-resolver";

import { AceLanguageClient } from "ace-linters/build/ace-language-client";

// to make sure language tools are loaded
ace.config.loadModule("ace/ext/language_tools");

import { Mode as TextMode } from "ace-builds/src-noconflict/mode-text";

export async function setupLspWorker() {
  // Create a web worker
  let worker = new Worker(new URL("./lspWorker.ts", import.meta.url), { name: "ic10lsp-Worker" });

  const loaded = (w: Worker) =>
    new Promise((r) => w.addEventListener("message", r, { once: true }));
  await Promise.all([loaded(worker)]);

  // Register the editor with the language provider
  return worker;
}

export import Ace = ace.Ace;
import { Range } from "ace-builds";

(ace as any).define("ace/marker_group", ["require", "exports", "module"], function (require: any, exports: any, module: any) {
  "use strict";
  var MarkerGroup = /** @class */ (function () {
    function MarkerGroup(session: any, options: any) {
      if (options)
        this.markerType = options.markerType;
      this.markers = [];
      this.session = session;
      session.addDynamicMarker(this);
    }
    MarkerGroup.prototype.getMarkerAtPosition = function (pos: any) {
      return this.markers.find(function (marker: any) {
        return marker.range.contains(pos.row, pos.column);
      });
    };
    MarkerGroup.prototype.markersComparator = function (a: any, b: any) {
      return a.range.start.row - b.range.start.row;
    };
    MarkerGroup.prototype.setMarkers = function (markers: any) {
      this.markers = markers.sort(this.markersComparator).slice(0, this.MAX_MARKERS);
      this.session._signal("changeBackMarker");
    };
    MarkerGroup.prototype.update = function (html: any, markerLayer: any, session: any, config: any) {
      if (!this.markers || !this.markers.length)
        return;
      var visibleRangeStartRow = config.firstRow, visibleRangeEndRow = config.lastRow;
      var foldLine;
      var markersOnOneLine = 0;
      var lastRow = 0;
      for (var i = 0; i < this.markers.length; i++) {
        var marker = this.markers[i];
        if (marker.range.end.row < visibleRangeStartRow)
          continue;
        if (marker.range.start.row > visibleRangeEndRow)
          continue;
        if (marker.range.start.row === lastRow) {
          markersOnOneLine++;
        }
        else {
          lastRow = marker.range.start.row;
          markersOnOneLine = 0;
        }
        if (markersOnOneLine > 200) {
          continue;
        }
        var markerVisibleRange = marker.range.clipRows(visibleRangeStartRow, visibleRangeEndRow);
        if (markerVisibleRange.start.row === markerVisibleRange.end.row
          && markerVisibleRange.start.column === markerVisibleRange.end.column) {
          continue; // visible range is empty
        }
        var screenRange = markerVisibleRange.toScreenRange(session);
        if (screenRange.isEmpty()) {
          foldLine = session.getNextFoldLine(markerVisibleRange.end.row, foldLine);
          if (foldLine && foldLine.end.row > markerVisibleRange.end.row) {
            visibleRangeStartRow = foldLine.end.row;
          }
          continue;
        }
        if (this.markerType === "fullLine") {
          markerLayer.drawFullLineMarker(html, screenRange, marker.className, config);
        }
        else if (screenRange.isMultiLine()) {
          if (this.markerType === "line")
            markerLayer.drawMultiLineMarker(html, screenRange, marker.className, config);
          else
            markerLayer.drawTextMarker(html, screenRange, marker.className, config);
        }
        else {
          markerLayer.drawSingleLineMarker(html, screenRange, marker.className + " ace_br15", config);
        }
      }
    };
    return MarkerGroup;
  }());
  MarkerGroup.prototype.MAX_MARKERS = 10000;
  exports.MarkerGroup = MarkerGroup;

});

export declare namespace AceHidden {
  export type Editor = Ace.Editor;
  export type EditSession = Ace.EditSession;
  export var popupManager: PopupManager;
  export class MouseEvent {
    constructor(domEvent: any, editor: any);
    /** @type {number} */ speed: number;
    /** @type {number} */ wheelX: number;
    /** @type {number} */ wheelY: number;
    domEvent: any;
    editor: any;
    x: any;
    clientX: any;
    y: any;
    clientY: any;
    $pos: any;
    $inSelection: any;
    propagationStopped: boolean;
    defaultPrevented: boolean;
    stopPropagation(): void;
    preventDefault(): void;
    stop(): void;
    /**
     * Get the document position below the mouse cursor
     *
     * @return {Object} 'row' and 'column' of the document position
     */
    getDocumentPosition(): Ace.Point;
    /**
     * Get the relative position within the gutter.
     *
     * @return {Number} 'row' within the gutter.
     */
    getGutterRow(): number;
    /**
     * Check if the mouse cursor is inside of the text selection
     *
     * @return {Boolean} whether the mouse cursor is inside of the selection
     */
    inSelection(): boolean;
    /**
     * Get the clicked mouse button
     *
     * @return {Number} 0 for left button, 1 for middle button, 2 for right button
     */
    getButton(): number;
    /**
     * @return {Boolean} whether the shift key was pressed when the event was emitted
     */
    getShiftKey(): boolean;
    getAccelKey(): any;
  }
  export class Tooltip {
    /**
     * @param {Element} parentNode
     **/
    constructor(parentNode: Element);
    isOpen: boolean;
    $element: any;
    $parentNode: Element;
    $init(): any;
    /**
     * @returns {HTMLElement}
     **/
    getElement(): HTMLElement;
    /**
     * @param {String} text
     **/
    setText(text: string): void;
    /**
     * @param {String} html
     **/
    setHtml(html: string): void;
    /**
     * @param {Number} x
     * @param {Number} y
     **/
    setPosition(x: number, y: number): void;
    /**
     * @param {String} className
     **/
    setClassName(className: string): void;
    /**
     * @param {import("../ace-internal").Ace.Theme} theme
     */
    setTheme(theme: any): void;
    /**
     * @param {String} [text]
     * @param {Number} [x]
     * @param {Number} [y]
     **/
    show(text?: string, x?: number, y?: number): void;
    hide(e: any): void;
    /**
     * @returns {Number}
     **/
    getHeight(): number;
    /**
     * @returns {Number}
     **/
    getWidth(): number;
    destroy(): void;
  }
  export class HoverTooltip extends Tooltip {
    constructor(parentNode?: HTMLElement);
    timeout: number;
    lastT: number;
    idleTime: number;
    lastEvent: any;
    onMouseOut(e: any): void;
    /**
     * @param {MouseEvent} e
     * @param {Editor} editor
     */
    onMouseMove(e: MouseEvent, editor: Editor): void;
    waitForHover(): void;
    /**
     * @param {Editor} editor
     */
    addToEditor(editor: Editor): void;
    /**
     * @param {Editor} editor
     */
    removeFromEditor(editor: Editor): void;
    /**
     * @param {MouseEvent} e
     */
    isOutsideOfText(e: MouseEvent): boolean;
    /**
     * @param {any} value
     */
    setDataProvider(value: (e: MouseEvent, editor: Ace.Editor) => void): void;
    $gatherData: (e: MouseEvent, editor: Ace.Editor) => void;
    /**
     * @param {Editor} editor
     * @param {Range} range
     * @param {any} domNode
     * @param {MouseEvent} startingEvent
     */
    showForRange(editor: Editor, range: Ace.Range, domNode: any, startingEvent: MouseEvent): void;
    range: any;
    /**
     * @param {Range} range
     * @param {EditSession} [session]
     */
    addMarker(range: Range, session?: EditSession): void;
    $markerSession: any;
    marker: any;
    $registerCloseEvents(): void;
    $removeCloseEvents(): void;
  }
  export class PopupManager {
    /**@type{Tooltip[]} */
    popups: Tooltip[];
    /**
     * @param {Tooltip} popup
     */
    addPopup(popup: Tooltip): void;
    /**
     * @param {Tooltip} popup
     */
    removePopup(popup: Tooltip): void;
    updatePopups(): void;
    /**
     * @param {Tooltip} popupA
     * @param {Tooltip} popupB
     * @return {boolean}
     */
    doPopupsOverlap(popupA: Tooltip, popupB: Tooltip): boolean;
  }
}

const { HoverTooltip } = ace.require("ace/tooltip");
const MarkerGroup = ace.require("ace/marker_group").MarkerGroup;

export { ace, TextMode, Range, AceLanguageClient, HoverTooltip, MarkerGroup }
