import { Offcanvas } from "bootstrap";
import { VirtualMachine, VirtualMachineUI } from ".";
import { DeviceRef, VM } from "ic10emu_wasm";




class VMDeviceUI {
  ui: VirtualMachineUI;
  summary: HTMLDivElement;
  canvasEl: HTMLDivElement;
  deviceCountEl: HTMLElement;
  canvas: Offcanvas;
  private _deviceSummaryCards: Map<number, VMDeviceSummaryCard>;
  private _offCanvaseCards: Map<
    number,
    { col: HTMLElement; card: VMDeviceCard }
  >;

  constructor(ui: VirtualMachineUI) {
    const that = this;
    that.ui = ui;
    this.summary = document.getElementById("vmDeviceSummary") as HTMLDivElement;
    this.canvasEl = document.getElementById(
      "vmDevicesOCBody",
    ) as HTMLDivElement;
    this.deviceCountEl = document.getElementById("vmViewDeviceCount");
    this.canvas = new Offcanvas(this.canvasEl);
    this._deviceSummaryCards = new Map();
    this._offCanvaseCards = new Map();
  }

  update(active_ic: DeviceRef) {
    const devices = window.VM.devices;
    this.deviceCountEl.textContent = `(${devices.size})`;
    for (const [id, device] of devices) {
      if (!this._deviceSummaryCards.has(id)) {
        this._deviceSummaryCards.set(id, new VMDeviceSummaryCard(this, device));
      }
      if (!this._offCanvaseCards.has(id)) {
        const col = document.createElement("div");
        col.classList.add("col");
        col.id = `${this.canvasEl.id}_col${id}`
        this.canvasEl.appendChild(col);
        this._offCanvaseCards.set(id, {
          col,
          card: new VMDeviceCard(this, col, device),
        });
      }
    }
    this._deviceSummaryCards.forEach((card, id, cards) => {
      if (!devices.has(id)) {
        card.destroy();
        cards.delete(id);
      } else {
        card.update(active_ic);
      }
    }, this);
    this._offCanvaseCards.forEach((card, id, cards) => {
      if (!devices.has(id)) {
        card.card.destroy();
        card.col.remove();
        cards.delete(id);
      } else {
        card.card.update(active_ic);
      }
    }, this);
  }
}

class VMDeviceSummaryCard {
  root: HTMLDivElement;
  viewBtn: HTMLButtonElement;
  deviceUI: VMDeviceUI;
  device: DeviceRef;
  badges: HTMLSpanElement[];
  constructor(deviceUI: VMDeviceUI, device: DeviceRef) {
    // const that = this;
    this.deviceUI = deviceUI;
    this.device = device;
    this.root = document.createElement("div");
    this.root.classList.add(
      "hstack",
      "gap-2",
      "bg-light-subtle",
      "border",
      "border-secondary-subtle",
      "rounded",
    );
    this.viewBtn = document.createElement("button");
    this.viewBtn.type = "button";
    this.viewBtn.classList.add("btn", "btn-success");
    this.root.appendChild(this.viewBtn);
    this.deviceUI.summary.appendChild(this.root);
    this.badges = [];

    this.update(window.VM.activeIC);
  }

  update(active_ic: DeviceRef) {
    const that = this;
    // clear previous badges
    this.badges.forEach((badge) => badge.remove());
    this.badges = [];

    //update name
    var deviceName = this.device.name ?? this.device.prefabName ?? "";
    if (deviceName) {
      deviceName = `: ${deviceName}`;
    }
    const btnTxt = `Device ${this.device.id}${deviceName}`;
    this.viewBtn.textContent = btnTxt;

    // regenerate badges
    this.device.connections.forEach((conn, index) => {
      if (typeof conn === "object") {
        var badge = document.createElement("span");
        badge.classList.add("badge", "text-bg-light");
        badge.textContent = `Net ${index}:${conn.CableNetwork}`;
        that.badges.push(badge);
        that.root.appendChild(badge);
      }
    });

    if (this.device.id === active_ic.id) {
      var badge = document.createElement("span");
      badge.classList.add("badge", "text-bg-success");
      badge.textContent = "db";
      that.badges.push(badge);
      that.root.appendChild(badge);
    }

    active_ic.pins?.forEach((id, index) => {
      if (that.device.id === id) {
        var badge = document.createElement("span");
        badge.classList.add("badge", "text-bg-success");
        badge.textContent = `d${index}`;
        that.badges.push(badge);
        that.root.appendChild(badge);
      }
    });
  }

  destroy() {
    this.root.remove();
  }
}

class VMDeviceCard {
  ui: VMDeviceUI;
  container: HTMLElement;
  device: DeviceRef;
  root: HTMLDivElement;
  nav: HTMLUListElement;

  header: HTMLDivElement;
  nameInput: HTMLInputElement;
  nameHash: HTMLSpanElement;
  body: HTMLDivElement;
  badges: HTMLSpanElement[];
  fieldsContainer: HTMLDivElement;
  slotsContainer: HTMLDivElement;
  pinsContainer: HTMLDivElement;
  networksContainer: HTMLDivElement;
  reagentsContainer: HTMLDivElement;
  nav_id: string;
  navTabs: { [key: string]: { li: HTMLLIElement; button: HTMLButtonElement } };
  paneContainer: HTMLDivElement;
  tabPanes: { [key: string]: HTMLElement };
  image: HTMLImageElement;
  image_err: boolean;
  title: HTMLHeadingElement;
  fieldEls: Map<string, VMDeviceField>;

  constructor(ui: VMDeviceUI, container: HTMLElement, device: DeviceRef) {
    this.ui = ui;
    this.container = container;
    this.device = device;
    this.nav_id = `${this.container.id}_vmDeviceCard${this.device.id}`;

    this.root = document.createElement("div");
    this.root.classList.add("card");

    this.header = document.createElement("div");
    this.header.classList.add("card-header", "hstack");
    this.image = document.createElement("img");
    this.image_err = false;
    this.image.src = `/img/stationpedia/${this.device.prefabName}.png`;
    this.image.onerror = this.onImageErr;
    this.image.width = 48;
    this.image.classList.add("me-2");
    this.header.appendChild(this.image);

    this.title = document.createElement("h5");
    this.title.textContent = `Device ${this.device.id} : ${this.device.prefabName ?? ""}`;
    this.header.appendChild(this.title);

    this.nameInput = document.createElement("input");
    this.nameHash = document.createElement("span");

    this.root.appendChild(this.header);

    this.body = document.createElement("div");
    this.body.classList.add("card-body");
    this.root.appendChild(this.body);

    this.nav = document.createElement("ul");
    this.nav.classList.add("nav", "nav-tabs");
    this.nav.role = "tablist";
    this.nav.id = this.nav_id;
    this.navTabs = {};
    this.tabPanes = {};

    this.body.appendChild(this.nav);

    this.paneContainer = document.createElement("div");
    this.paneContainer.id = `${this.nav_id}_tabs`;

    this.body.appendChild(this.paneContainer);

    this.badges = [];
    this.fieldsContainer = document.createElement("div");
    this.fieldsContainer.id = `${this.nav_id}_fields`;
    this.fieldsContainer.classList.add("vstack");
    this.fieldEls = new Map();
    this.slotsContainer = document.createElement("div");
    this.slotsContainer.id = `${this.nav_id}_slots`;
    this.slotsContainer.classList.add("vstack");
    this.reagentsContainer = document.createElement("div");
    this.reagentsContainer.id = `${this.nav_id}_reagents`;
    this.reagentsContainer.classList.add("vstack");
    this.networksContainer = document.createElement("div");
    this.networksContainer.id = `${this.nav_id}_networks`;
    this.networksContainer.classList.add("vstack");
    this.pinsContainer = document.createElement("div");
    this.pinsContainer.id = `${this.nav_id}_pins`;
    this.pinsContainer.classList.add("vstack");

    this.addTab("Fields", this.fieldsContainer);
    this.addTab("Slots", this.slotsContainer);
    this.addTab("Networks", this.networksContainer);

    this.update(window.VM.activeIC);

    // do last to minimise reflows
    this.container.appendChild(this.root);
  }

  onImageErr(e: Event) {
    this.image_err = true;
    console.log("Image load error", e);
  }

  addNav(name: string, target: string) {
    if (!(name in this.navTabs)) {
      var li = document.createElement("li");
      li.classList.add("nav-item");
      li.role = "presentation";
      var button = document.createElement("button");
      button.classList.add("nav-link");
      if (!(Object.keys(this.navTabs).length > 0)) {
        button.classList.add("active");
        button.tabIndex = 0;
      } else {
        button.tabIndex = -1;
      }
      button.id = `${this.nav_id}_tab_${name}`;
      button.setAttribute("data-bs-toggle", "tab");
      button.setAttribute("data-bs-target", `#${target}`);
      button.type = "button";
      button.role = "tab";
      button.setAttribute("aria-controls", target);
      button.setAttribute(
        "aria-selected",
        Object.keys(this.navTabs).length > 0 ? "false" : "true",
      );
      button.textContent = name;
      li.appendChild(button);
      this.nav.appendChild(li);
      this.navTabs[name] = { li, button };
      return true;
    }
    return false;
  }

  removeNav(name: string) {
    if (name in this.navTabs) {
      this.navTabs[name].li.remove();
      delete this.navTabs[name];
      return true;
    }
    return false;
  }

  addTab(name: string, tab: HTMLElement) {
    const paneName = `${this.nav_id}_pane_${name}`;
    if (this.addNav(name, paneName)) {
      if (name in this.tabPanes) {
        this.tabPanes[name].remove();
      }
      const pane = document.createElement("div");
      pane.classList.add("tap-pane", "fade");
      if (!(Object.keys(this.tabPanes).length > 0)) {
        pane.classList.add("show", "active");
      }
      pane.id = paneName;
      pane.role = "tabpanel";
      pane.setAttribute("aria-labelledby", `${this.nav_id}_tab_${name}`);
      pane.tabIndex = 0;

      this.paneContainer.appendChild(pane);
      pane.appendChild(tab);
      this.tabPanes[name] = tab;
    }
  }

  removeTab(name: string) {
    let result = this.removeNav(name);
    if (name in this.tabPanes) {
      this.tabPanes[name].remove();
      delete this.tabPanes[name];
      return true;
    }
    return result;
  }

  update(active_ic: DeviceRef) {
    if (this.device.pins) {
      this.addTab("Pins", this.pinsContainer);
    } else {
      this.removeTab("Pins");
    }

    // fields
    for (const [name, _field] of this.device.fields) {
      if (!this.fieldEls.has(name)) {
        const field = new VMDeviceField(this.device, name, this, this.fieldsContainer);
        this.fieldEls.set(name, field);
      }
    }
    this.fieldEls.forEach((field, name, map) => {
      if(!this.device.fields.has(name)) {
        field.destroy();
        map.delete(name);
      } else {
        field.update(active_ic);
      }
    }, this);


    // TODO Reagents
  }

  destroy() {
    this.root.remove();
  }
}

class VMDeviceField {
  container: HTMLElement;
  card: VMDeviceCard;
  device: DeviceRef;
  field: string;
  root: HTMLDivElement;
  name: HTMLSpanElement;
  fieldType: HTMLSpanElement;
  input: HTMLInputElement;
  constructor(device: DeviceRef, field: string, card: VMDeviceCard, container: HTMLElement) {
    this.device = device;
    this.field = field;
    this.card = card;
    this.container = container;
    this.root = document.createElement('div');
    this.root.classList.add("input-group", "input-group-sm");
    this.name = document.createElement('span');
    this.name.classList.add("input-group-text", "field_name");
    this.name.textContent = this.field;
    this.root.appendChild(this.name);
    this.fieldType = document.createElement('span');
    this.fieldType.classList.add("input-group-text", "field_type");
    this.fieldType.textContent = device.fields.get(this.field)?.field_type;
    this.root.appendChild(this.fieldType);
    this.input = document.createElement('input');
    this.input.type = "text";
    this.input.value = this.device.fields.get(this.field)?.value.toString();
    this.root.appendChild(this.input);

    this.container.appendChild(this.root);
  }
  destroy () {
    this.root.remove();
  }
  update(_active_ic: DeviceRef) {
    this.input.value = this.device.fields.get(this.field)?.value.toString();
  }
}

export { VMDeviceUI };
