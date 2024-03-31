import { Offcanvas } from 'bootstrap';
import { VirtualMachine, VirtualMachineUI } from '.';
import { DeviceRef } from 'ic10emu_wasm';


class VMDeviceUI {
    ui: VirtualMachineUI;
    root: HTMLDivElement;
    canvasEl: HTMLElement;
    deviceCountEl: HTMLElement;
    canvas: Offcanvas;

    constructor(ui: VirtualMachineUI) {
        const that = this;
        that.ui = ui;
        this.root = document.createElement('div');
        this.canvasEl = document.getElementById('vmDevicesOCBody');
        this.deviceCountEl = document.getElementById('vmDViewDeviceCount');
        this.canvas = new Offcanvas(this.canvasEl)

    }

    update(active_ic: DeviceRef) {
        const devices = window.VM.devices;


    }

}

class VMDeviceCard {
    root: HTMLDivElement;
    viewBtn: HTMLButtonElement;
    deviceUI: VMDeviceUI;
    device: DeviceRef;
    constructor(deviceUI: VMDeviceUI, device: DeviceRef) {
        const that = this;
        this.deviceUI = deviceUI;
        this.device = device;
        this.root = document.createElement('div');
        this.root.classList.add("hstack", "gap-2");
        this.viewBtn = document.createElement('button');
        this.viewBtn.type = "button";
        this.viewBtn.classList.add("btn", "btn-secondary");
        const btnTxt = document.createTextNode(device.name)
        this.deviceUI.root.appendChild(this.root);
    }

    destroy() {
        this.root.remove();
    }

}

export { VMDeviceUI }