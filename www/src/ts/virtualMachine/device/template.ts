import type {
  Connection,
  ObjectTemplate,
  LogicField,
  LogicType,
  Slot,
  Class,
  FrozenObject,
  MemoryAccess,
  SlotInfo,
  ConnectionInfo,
  ObjectID,
  CableConnectionType,
  ConnectionRole,
  ObjectInfo,
  SlotOccupantInfo,
} from "ic10emu_wasm";
import { html, css, HTMLTemplateResult } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import { BaseElement, defaultCss } from "components";

import { connectionFromConnectionInfo } from "./dbutils";
import { crc32, displayNumber, parseNumber } from "utils";
import SlInput from "@shoelace-style/shoelace/dist/components/input/input.component.js";
import SlSelect from "@shoelace-style/shoelace/dist/components/select/select.component.js";
import { VMDeviceCard } from "./card";
import { globalObjectSignalMap, VMTemplateDBMixin } from "virtualMachine/baseDevice";
import { computed, Signal, watch } from "@lit-labs/preact-signals";
import { createRef, ref, Ref } from "lit/directives/ref.js";

export interface SlotTemplate {
  typ: Class;
  quantity: number;
  occupant?: FrozenObject;
}

export interface ConnectionCableNetwork {
  CableNetwork: {
    net: ObjectID | undefined;
    typ: CableConnectionType;
    role: ConnectionRole;
  };
}

@customElement("vm-device-template")
export class VmObjectTemplate extends VMTemplateDBMixin(BaseElement) {
  static styles = [
    ...defaultCss,
    css`
      .template-card {
        --padding: var(--sl-spacing-small);
      }
      .image {
        width: 3rem;
        height: 3rem;
      }
      .header {
        display: flex;
        flex-direction: row;
      }
      .card-body {
        // height: 18rem;
        overflow-y: auto;
      }
      sl-tab-group {
        --indicator-color: var(--sl-color-purple-600);
        --sl-color-primary-600: var(--sl-color-purple-600);
      }
      sl-tab::part(base) {
        padding: var(--sl-spacing-small) var(--sl-spacing-medium);
      }
      sl-tab-group::part(base) {
        height: 18rem;
        overflow-y: auto;
      }
    `,
  ];

  fields: Signal<Record<LogicType, number>>;
  slots: Signal<SlotTemplate[]>;
  pins: Signal<(ObjectID | undefined)[]>;
  template: Signal<FrozenObject>;
  objectId: Signal<number | undefined>;
  objectName: Signal<string | undefined>;
  connections: Signal<Connection[]>;

  constructor() {
    super();
    this.templateDB = window.VM.vm.templateDB;
  }

  private _prefabName: string;
  private _prefabHash: number;

  get prefabName(): string {
    return this._prefabName;
  }
  get prefabHash(): number {
    return this._prefabHash;
  }

  @property({ type: String })
  set prefabName(val: string) {
    this._prefabName = val;
    this._prefabHash = crc32(this._prefabName);
    this.setupState();
  }

  get dbTemplate(): ObjectTemplate {
    return this.templateDB.get(this._prefabHash);
  }

  setupState() {
    const dbTemplate = this.dbTemplate;

    this.fields.value = Object.fromEntries(
      (
        Array.from(
          "logic" in dbTemplate
            ? Object.entries(dbTemplate.logic.logic_types)
            : [],
        ) as [LogicType, MemoryAccess][]
      ).map(([lt, access]) => {
        const value =
          lt === "PrefabHash" ? this.dbTemplate.prefab.prefab_hash : 0.0;
        return [lt, value];
      }),
    ) as Record<LogicType, number>;

    this.slots.value = (
      ("slots" in dbTemplate ? dbTemplate.slots ?? [] : []) as SlotInfo[]
    ).map(
      (slot, _index) =>
        ({
          typ: slot.typ,
          quantity: 0,
        }) as SlotTemplate,
    );

    const connections = (
      "device" in dbTemplate
        ? dbTemplate.device.connection_list
        : ([] as ConnectionInfo[])
    ).map(
      (conn, index) => [index, connectionFromConnectionInfo(conn)] as const,
    );
    connections.sort((a, b) => {
      if (a[0] < b[0]) {
        return -1;
      } else if (a[0] > b[0]) {
        return 1;
      } else {
        return 0;
      }
    });

    this.connections.value = connections.map((conn) => conn[1]);

    const numPins =
      "device" in dbTemplate ? dbTemplate.device.device_pins_length : 0;
    this.pins.value = new Array(numPins).fill(undefined);
  }

  renderFields(): HTMLTemplateResult {
    const fields = Object.entries(this.fields);
    return html`
      ${fields.map(([name, field], _index, _fields) => {
      return html`
          <sl-input
            key="${name}"
            value="${displayNumber(field.value)}"
            size="small"
            @sl-change=${this._handleChangeField}
            ?disabled=${name === "PrefabHash"}
          >
            <span slot="prefix">${name}</span>
            <span slot="suffix">${field.field_type}</span>
          </sl-input>
        `;
    })}
    `;
  }

  _handleChangeField(e: CustomEvent) {
    const input = e.target as SlInput;
    const field = input.getAttribute("key")! as LogicType;
    const val = parseNumber(input.value);
    this.fields.value = { ...this.fields.value, [field]: val};
    if (field === "ReferenceId" && val !== 0) {
      this.objectId.value = val;
    }
  }

  forceSelectUpdate(...slSelects: Ref<SlSelect>[]) {
    for (const slSelect of slSelects) {
      if (slSelect.value != null && "handleValueChange" in slSelect.value) {
        slSelect.value.handleValueChange();
      }
    }
  }

  private networksSelectRef: Ref<SlSelect> = createRef();

  renderSlot(slot: Slot, slotIndex: number): HTMLTemplateResult {
    return html`<sl-card class="slot-card"> </sl-card>`;
  }

  renderSlots(): HTMLTemplateResult {
    return html`<div clas="slots"></div>`;
  }

  renderReagents(): HTMLTemplateResult {
    return html``;
  }

  renderNetworks() {
    const vm = window.VM.vm;
    const vmNetworks = computed(() => {
      return vm.networkIds.value.map((net) => html`<sl-option value=${net}>Network ${net}</sl-option>`);
    });
    const connections = computed(() => {
      this.connections.value.map((connection, index, _conns) => {
        const conn =
          typeof connection === "object" && "CableNetwork" in connection
            ? connection.CableNetwork
            : null;
        return html`
          <sl-select
            hoist
            placement="top"
            clearable
            key=${index}
            value=${conn?.net}
            ?disabled=${conn === null}
            @sl-change=${this._handleChangeConnection}
            ${ref(this.networksSelectRef)}
          >
            <span slot="prefix">Connection:${index} </span>
            ${watch(vmNetworks)}
            <span slot="prefix"> ${conn?.typ} </span>
          </sl-select>
        `;
      });
    });
    vmNetworks.subscribe((_) => { this.forceSelectUpdate(this.networksSelectRef)})
    return html`
      <div class="networks">
        ${watch(connections)}
      </div>
    `;
  }

  _handleChangeConnection(e: CustomEvent) {
    const select = e.target as SlSelect;
    const conn = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : undefined;
    const copy = [...this.connections.value];
    (copy[conn] as ConnectionCableNetwork).CableNetwork.net = val;
    this.connections.value = copy;
  }

  private _pinsSelectRefMap: Map<number, Ref<SlSelect>> = new Map();

  getPinRef(index: number) : Ref<SlSelect> {
    if (!this._pinsSelectRefMap.has(index)) {
      this._pinsSelectRefMap.set(index, createRef());
    }
    return this._pinsSelectRefMap.get(index);
  }

  forcePinSelectUpdate() {
    this.forceSelectUpdate(...this._pinsSelectRefMap.values());
  }

  renderPins(): HTMLTemplateResult {
    const networks = computed(() => {
      return this.connections.value.flatMap((connection, index) => {
        return typeof connection === "object" && "CableNetwork" in connection
          ? [connection.CableNetwork.net]
          : [];
      });
    });
    const visibleDeviceIds = computed(() => {
      return  [
      ...new Set(
        networks.value.flatMap((net) => window.VM.vm.networkDataDevicesSignal(net).value),
      ),
    ];

    });
    const visibleDevices = computed(() => {
      return visibleDeviceIds.value.map((id) =>
        globalObjectSignalMap.get(id),
      );
    });
    const visibleDevicesHtml = computed(() => {
      return visibleDevices.value.map(
            (device, _index) => {
              device.id.subscribe((_) => { this.forcePinSelectUpdate(); });
              device.displayName.subscribe((_) => { this.forcePinSelectUpdate(); });
              return html`
                <sl-option value=${watch(device.id)}>
                  Device ${watch(device.id)} :
                  ${watch(device.displayName)}
                </sl-option>
              `
            }
          )
    });
    visibleDeviceIds.subscribe((_) => { this.forcePinSelectUpdate(); });
    const pinsHtml = computed(() => {
      this.pins.value.map(
        (pin, index) => {
          const pinRef = this.getPinRef(index)
          return html` <sl-select
            hoist
            placement="top"
            clearable
            key=${index}
            .value=${pin}
            @sl-change=${this._handleChangePin}
            ${ref(pinRef)}
          >
            <span slot="prefix">d${index}</span>
            ${watch(visibleDevicesHtml)}
          </sl-select>`
        }
      );
    });
    return html`<div class="pins">${watch(pinsHtml)}</div>`;
  }

  _handleChangePin(e: CustomEvent) {
    const select = e.target as SlSelect;
    const pin = parseInt(select.getAttribute("key")!);
    const val = select.value ? parseInt(select.value as string) : null;
    const copy = [...this.pins.value];
    copy[pin] = val;
    this.pins.value = copy;
  }

  render() {
    const device = this.dbTemplate;
    return html`
      <sl-card class="template-card">
        <div class="header h-20 w-96" slot="header">
          <sl-tooltip content="${device?.prefab.prefab_name}">
            <img
              class="image me-2"
              src="img/stationpedia/${device?.prefab.prefab_name}.png"
              onerror="this.src = '${VMDeviceCard.transparentImg}'"
            />
          </sl-tooltip>
          <div class="vstack">
            <span>${device.prefab.name}</span>
            <span><small>${device?.prefab.prefab_name}</small></span>
            <span><small>${device?.prefab.prefab_hash}</small></span>
          </div>
          <sl-button
            class="ms-auto mt-auto mb-auto"
            pill
            variant="success"
            @click=${this._handleAddButtonClick}
            >Add <sl-icon slot="prefix" name="plus-lg"></sl-icon>
          </sl-button>
        </div>
        <div class="card-body">
          <sl-tab-group>
            <sl-tab slot="nav" panel="fields">Fields</sl-tab>
            <sl-tab slot="nav" panel="slots">Slots</sl-tab>
            <!-- <sl-tab slot="nav" panel="reagents">Reagents</sl-tab> -->
            <sl-tab slot="nav" panel="networks">Networks</sl-tab>
            <!-- <sl-tab slot="nav" panel="pins">Pins</sl-tab> -->

            <sl-tab-panel name="fields">${this.renderFields()}</sl-tab-panel>
            <sl-tab-panel name="slots">${this.renderSlots()}</sl-tab-panel>
            <!-- <sl-tab-panel name="reagents">${this.renderReagents()}</sl-tab-panel> -->
            <sl-tab-panel name="networks"
              >${this.renderNetworks()}</sl-tab-panel
            >
            <!-- <sl-tab-panel name="pins">${this.renderPins()}</sl-tab-panel> -->
          </sl-tab-group>
        </div>
      </sl-card>
    `;
  }
  async _handleAddButtonClick() {
    this.dispatchEvent(
      new CustomEvent("add-device-template", { bubbles: true }),
    );
    // Typescript doesn't like  fileds defined as  `X | undefined` not being present, hence cast
    const objInfo: ObjectInfo = {
      id: this.objectId.value,
      name: this.objectName.value,
      prefab: this.prefabName,
    } as ObjectInfo;

    if (this.slots.value.length > 0) {
      const slotOccupants: [FrozenObject, number][] = this.slots.value.flatMap(
        (slot, index) => {
          return typeof slot.occupant !== "undefined"
            ? [[slot.occupant, index]]
            : [];
        },
      );
      let slotOccupantTemplates: FrozenObject[] | null = null;
      let slotOccupantObjectIds: ObjectID[] | null = null;

      let slotOccupantIdsMap: Map<number, number> = new Map();
      if (slotOccupants.length > 0) {
        slotOccupantTemplates = slotOccupants.map(([slot, _]) => slot);
        slotOccupantObjectIds = await window.VM.vm.addObjectsFrozen(
          slotOccupantTemplates,
        );
        slotOccupantIdsMap = new Map(
          slotOccupants.map((_, index) => {
            return [index, slotOccupantObjectIds[index]];
          }),
        );
      }
      objInfo.slots = new Map(
        this.slots.value.flatMap((slot, index) => {
          const occupantId = slotOccupantIdsMap.get(index);
          if (typeof occupantId !== "undefined") {
            const info: SlotOccupantInfo = {
              id: occupantId,
              quantity: slot.quantity,
            };
            return [[index, info]] as [number, SlotOccupantInfo][];
          } else {
            return [] as [number, SlotOccupantInfo][];
          }
        }),
      );
    }

    if (this.connections.value.length > 0) {
      objInfo.connections = new Map(
        this.connections.value.flatMap((conn, index) => {
          return typeof conn === "object" &&
            "CableNetwork" in conn &&
            typeof conn.CableNetwork.net !== "undefined"
            ? ([[index, conn.CableNetwork.net]] as [number, number][])
            : ([] as [number, number][]);
        }),
      );
    }

    if (Object.keys(this.fields.value).length > 0) {
      objInfo.logic_values = new Map(Object.entries(this.fields.value) as [LogicType, number][]);
    }

    const template: FrozenObject = {
      obj_info: objInfo,
      database_template: true,
      template: undefined,
    };
    await window.VM.vm.addObjectFrozen(template);

    // reset state for new device
    this.setupState();
  }
}
