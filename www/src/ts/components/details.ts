import {
  html,
  css,
  HTMLTemplateResult,
  PropertyValueMap,
  CSSResultGroup,
} from "lit";
import { customElement, query, state } from "lit/decorators.js";
import { classMap } from "lit/directives/class-map.js";
import SlDetails from "@shoelace-style/shoelace/dist/components/details/details.js";

@customElement("ic10-details")
export class IC10Details extends SlDetails {
  @query(".details__summary-icon") accessor summaryIcon: HTMLSpanElement;

  static styles = [
    SlDetails.styles,
    css`
      .details__header {
        cursor: auto;
      }
      .details__summary-icon {
        cursor: pointer;
      }
    `,
  ];

  constructor() {
    super();
  }

  private handleSummaryIconClick(event: MouseEvent) {
    event.preventDefault();

    if (!this.disabled) {
      if (this.open) {
        this.hide();
      } else {
        this.show();
      }
      this.header.focus();
    }
  }

  private handleSummaryIconKeyDown(event: KeyboardEvent) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();

      if (this.open) {
        this.hide();
      } else {
        this.show();
      }
    }

    if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      this.hide();
    }

    if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      this.show();
    }
  }

  render() {
    return html`
      <details part="base" class=${classMap({ details: true, "details--open" : this.open, "details--disabled" : this.disabled,
        })}>
        <summary part="header" id="header" class="details__header" role="button" aria-expanded=${this.open ? "true" : "false"
          } aria-controls="content" aria-disabled=${this.disabled ? "true" : "false" } tabindex=${this.disabled ? "-1" : "0" }
          @click=${(e: Event)=> e.preventDefault()}
          >
          <slot name="summary" part="summary" class="details__summary">${this.summary}</slot>

          <span part="summary-icon" class="details__summary-icon" @click=${this.handleSummaryIconClick}
            @keydown=${this.handleSummaryIconKeyDown}>
            <slot name="expand-icon">
              <sl-icon library="system" name="chevron-right"></sl-icon>
            </slot>
            <slot name="collapse-icon">
              <sl-icon library="system" name="chevron-right"></sl-icon>
            </slot>
          </span>
        </summary>

        <div class="details__body" role="region" aria-labelledby="header">
          <slot part="content" id="content" class="details__content"></slot>
        </div>
      </details>
    `;
  }
}
