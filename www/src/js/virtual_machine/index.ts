import { init } from "ic10emu_wasm";
import { VMDeviceUI } from "./device";
// import { Card } from 'bootstrap';

class VirtualMachine {

    constructor() {
        const vm = init();

        window.VM = this;

        this.ic10vm = vm;
        this.ui = new VirtualMachineUI(this);

        this._devices = new Map();
        this._ics = new Map();

        this.updateDevices();

        this.updateCode()

    }

    get devices () {
        return this._devices;
    }

    get ics () {
        return this._ics;
    }

    get activeIC () {
        return this._ics.get(window.App.session.activeSession);
    }

    updateDevices() {
        
        const device_ids = this.ic10vm.devices;
        for (const id of device_ids) {
            if (!this._devices.has(id)){
                this._devices.set(id, this.ic10vm.getDevice(id));
            }
        }
        for(const id of this._devices) {
            if (!device_ids.includes(id)) {
                this._devices.delete(id);
            }
        }

        const ics = this.ic10vm.ics;
        for (const id of ics) {
            if (!this._ics.has(id)){
                this._ics.set(id, this._devices.get(id));
            }
        }
        for(const id of this._ics) {
            if (!ics.includes(id)) {
                this._ics.delete(id);
            }
        }

    }

    updateCode() {
        const progs = window.App.session.programs;
        for (const id of Object.keys(progs)) {
            const attempt = Date.now().toString(16)
            const ic = this._ics.get(id);
            const prog = progs[id];
            if (ic && prog) {
                console.time(`CompileProgram_${id}_${attempt}`);
                try {
                    this.ics[id].setCode(progs[id]);
                } catch (e) {
                    console.log(e);
                }                
                console.timeEnd(`CompileProgram_${id}_${attempt}`);
            }
        }
        this.update();
    }

    step() {
        const ic = this.activeIC;
        if (ic) {
            try {
                ic.step();
            } catch (e) {
                console.log(e);
            }
            this.update();
        }
    }

    run() {
        const ic = this.activeIC;
        if (ic) {
            try {
                ic.run(false);
            } catch (e) {
                console.log(e);
            }
            this.update();
        }
    }

    reset() {
        const ic = this.activeIC;
        if (ic) {
            ic.reset();
            this.update();
        }
    }

    update() {
        this.updateDevices();
        const ic = this.activeIC;
        window.App.session.setActiveLine(window.App.session.activeSession, ic.ip);
        this.ui.update(ic);
    }

    setRegister(index, val) {
        const ic = this.activeIC;
        try {
            ic.setRegister(index, val);
        } catch (e) {
            console.log(e);
        }
    }

    setStack(addr, val) {
        const ic = this.activeIC;
        try {
            ic.setStack(addr, val);
        } catch (e) {
            console.log(e);
        }
    }

    setupDeviceDatabase(db) {
        this.db = db;
        console.log("Loaded Device Database", this.db);
    }
}


class VirtualMachineUI {
    constructor(vm) {
        this.vm = vm
        this.state = new VMStateUI(this);
        this.registers = new VMRegistersUI(this);
        this.stack = new VMStackUI(this);
        this.devices = new VMDeviceUI(this);

        const that = this;

        document.getElementById("vmControlRun").addEventListener('click', (_event) => {
            that.vm.run();
        }, { capture: true });
        document.getElementById("vmControlStep").addEventListener('click', (_event) => {
            that.vm.step();
        }, { capture: true });
        document.getElementById("vmControlReset").addEventListener('click', (_event) => {
            that.vm.reset();
        }, { capture: true });

    }

    update(ic) {
        this.state.update(ic);
        this.registers.update(ic);
        this.stack.update(ic);
        this.devices.update(ic);
    }

}

class VMStateUI {
    constructor(ui) {
        this.ui = ui;

        this.instructionPointer = document.getElementById("vmActiveICStateIP");
        this.instructionCounter = document.getElementById("vmActiveICStateICount");
        this.lastState = document.getElementById("vmActiveICStateLastRun");
    }

    update(ic) {
        if (ic) {
            this.instructionPointer.innerText = ic.ip.toString();
            this.instructionCounter.innerText = ic.instructionCount.toString();
            this.lastState.innerText = ic.state.toString();
        }
    }
}

class VMRegistersUI {
    constructor(ui) {
        this.ui = ui;
        const regDom = document.getElementById("vmActiveRegisters");
        this.tbl = document.createElement("div");
        this.tbl.classList.add("d-flex", "flex-wrap", "justify-content-start", "align-items-end",);
        this.regCells = [];
        for (var i = 0; i < 18; i++) {
            const container = document.createElement("div");
            container.classList.add("vm_reg_cel", "align-that-stretch");
            const cell = document.createElement("div");
            cell.classList.add("input-group", "input-group-sm")
            // cell.style.width = "30%";
            const nameLabel = document.createElement("span");
            nameLabel.innerText = `r${i}`;
            nameLabel.classList.add("input-group-text")
            cell.appendChild(nameLabel);
            const input = document.createElement("input");
            input.type = "text"
            input.value = 0;
            input.dataset.index = i;
            cell.appendChild(input);
            const aliasesLabel = document.createElement("span");
            aliasesLabel.classList.add("input-group-text", "reg_label")
            cell.appendChild(aliasesLabel);
            this.regCells.push({
                cell,
                nameLabel,
                aliasesLabel,
                input,
            });
            container.appendChild(cell);
            this.tbl.appendChild(container);
        }
        this.regCells.forEach(cell => {
            cell.input.addEventListener('change', this.onCellUpdate);
        });
        this.default_aliases = { "sp": 16, "ra": 17 }
        this.ic_aliases = {}
        regDom.appendChild(this.tbl);
    }

    onCellUpdate(e) {
        let index;
        let val;
        try {
            index = parseInt(e.target.dataset.index);
            val = parseFloat(e.target.value);
        } catch (e) {
            // reset the edit
            console.log(e);
            VM.update();
            return;
        }
        VM.setRegister(index, val);
    }

    update(ic) {
        const that = this;
        if (ic) {
            const registers = ic.registers;
            if (registers) {
                for (var i = 0; i < registers.length; i++) {
                    this.regCells[i].input.value = registers[i];
                }
            }
            const aliases = ic.aliases;
            if (aliases) {
                this.ic_aliases = {};
                aliases.forEach( (target, alias, _map) => {
                    // const target = aliases.get(alias);
                    if (target.RegisterSpec &&  target.RegisterSpec.indirection == 0) {
                        const index = target.RegisterSpec.target;
                        this.ic_aliases[alias] = index;
                    }
                })
            }
            console.log(aliases);
        }
        this.updateAliases();
    }

    updateAliases () {
        const aliases = Object.assign({}, this.default_aliases, this.ic_aliases);
        const labels = {}
        for (const [alias, target] of Object.entries(aliases)) {
            if (labels.hasOwnProperty(target)) {
                labels[target].push(alias)
            } else {
                labels[target] = [alias]
            }
        }

        for(const [index, label] of Object.entries(labels)) {
            this.regCells[index].aliasesLabel.innerText = label.join(", ")
        }
    }
}

class VMStackUI {
    constructor(ui) {
        this.ui = ui;
        const stackDom = document.getElementById("vmActiveStack");
        this.tbl = document.createElement("div");
        this.tbl.classList.add("d-flex", "flex-wrap", "justify-content-start", "align-items-end",);
        this.stackCells = [];
        for (var i = 0; i < 512; i++) {
            const container = document.createElement("div");
            container.classList.add("vm_stack_cel", "align-that-stretch");
            const cell = document.createElement("div");
            cell.classList.add("input-group", "input-group-sm")
            const nameLabel = document.createElement("span");
            nameLabel.innerText = `${i}`;
            nameLabel.classList.add("input-group-text")
            cell.appendChild(nameLabel);
            const input = document.createElement("input");
            input.type = "text"
            input.value = 0;
            input.dataset.index = i;
            cell.appendChild(input);
            
            this.stackCells.push({
                cell,
                nameLabel,
                input,
            });
            container.appendChild(cell);
            this.tbl.appendChild(container);
        }
        this.stackCells.forEach(cell => {
            cell.input.addEventListener('change', this.onCellUpdate);
        });
        stackDom.appendChild(this.tbl);
    }

    onCellUpdate(e) {
        let index;
        let val;
        try {
            index = parseInt(e.target.dataset.index);
            val = parseFloat(e.target.value);
        } catch (e) {
            // reset the edit
            console.log(e);
            VM.update();
            return;
        }
        VM.setStack(index, val);
    }

    update(ic) {
        const that = this;
        if (ic) {
            const stack = ic.stack;
            const sp = ic.registers[16];
            if (stack) {
                for (var i = 0; i < stack.length; i++) {
                    this.stackCells[i].input.value = stack[i];
                    if (i == sp) {
                        this.stackCells[i].nameLabel.classList.add("stack_pointer");
                    } else {
                        this.stackCells[i].nameLabel.classList.remove("stack_pointer");
                    }
                }
            }
        }
    }

}

export { VirtualMachine }