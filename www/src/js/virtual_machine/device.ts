import { Offcanvas } from 'bootstrap';
import { VirtualMachine, VirtualMachineUI } from '.';
import { DeviceRef, VM } from 'ic10emu_wasm';


class VMDeviceUI {
    ui: VirtualMachineUI;
    summary: HTMLDivElement;
    canvasEl: HTMLDivElement;
    deviceCountEl: HTMLElement;
    canvas: Offcanvas;
    private _deviceSummaryCards: Map<number, VMDeviceSummaryCard>;

    constructor(ui: VirtualMachineUI) {
        const that = this;
        that.ui = ui;
        this.summary = document.getElementById('vmDeviceSummary') as HTMLDivElement;
        this.canvasEl = document.getElementById('vmDevicesOCBody') as HTMLDivElement;
        this.deviceCountEl = document.getElementById('vmViewDeviceCount');
        this.canvas = new Offcanvas(this.canvasEl);
        this._deviceSummaryCards = new Map();
    }

    update(active_ic: DeviceRef) {
        const devices = window.VM.devices;
        this.deviceCountEl.innerText = `(${devices.size})`
        for (const [id, device] of devices) {
            if (!this._deviceSummaryCards.has(id)) {
                this._deviceSummaryCards.set(id, new VMDeviceSummaryCard(this, device));
            }
        }
        this._deviceSummaryCards.forEach((card, _id) => { card.update(active_ic)});
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
        this.root = document.createElement('div');
        this.root.classList.add("hstack", "gap-2", "bg-light-subtle", "border", "border-secondary-subtle", "rounded");
        this.viewBtn = document.createElement('button');
        this.viewBtn.type = "button";
        this.viewBtn.classList.add("btn", "btn-success" );
        this.root.appendChild(this.viewBtn);
        this.deviceUI.summary.appendChild(this.root);
        this.badges = [];

        this.update(window.VM.activeIC);
    }

    update (active_ic: DeviceRef) {

        const that = this;
        // clear previous badges
        this.badges.forEach(badge => badge.remove());
        this.badges = []

        //update name
        var deviceName = this.device.name ?? this.device.prefabName ?? "";
        if (deviceName) {
            deviceName = `: ${deviceName}`
        }
        const btnTxt = `Device ${this.device.id}${deviceName}`
        this.viewBtn.innerText = btnTxt;

        // regenerate badges
        this.device.connections.forEach((conn, index) => {
            if ( typeof conn === "object") {
                var badge = document.createElement('span');
                badge.classList.add("badge", "text-bg-light");
                badge.innerText = `Net ${index}:${conn.CableNetwork}`;
                that.badges.push(badge);
                that.root.appendChild(badge);
            }

        });

        if (this.device.id === active_ic.id) {
            var badge = document.createElement('span');
            badge.classList.add("badge", "text-bg-success");
            badge.innerText = "db";
            that.badges.push(badge);
            that.root.appendChild(badge);
        }

        active_ic.pins?.forEach((id, index) => {
            if (that.device.id === id) {
                var badge = document.createElement('span');
                badge.classList.add("badge", "text-bg-success");
                badge.innerText = `d${index}`;
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
    root: HTMLDivElement;

    header: HTMLHeadingElement;
    device: DeviceRef;
    nameInput: HTMLInputElement;
    nameHash: HTMLSpanElement;
    badges: HTMLSpanElement[];
    fieldsContainer: HTMLDivElement;
    slotsContainer: HTMLDivElement;
    pinsContainer: HTMLDivElement;
    networksContainer: HTMLDivElement;

    constructor(ui: VMDeviceUI, container: HTMLElement, device: DeviceRef) {
        this.ui = ui;
        this.container = container;
        this.device = device;

        this.root = document.createElement('div');

        this.header = document.createElement('h5');
        this.nameInput = document.createElement('input');
        this.nameHash = document.createElement('span');
        this.badges = [];
        this.fieldsContainer = document.createElement('div');
        this.slotsContainer = document.createElement('div');
        this.pinsContainer = document.createElement('div');
        this.networksContainer = document.createElement('div');
    }
}

export { VMDeviceUI }
