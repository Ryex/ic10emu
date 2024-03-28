import { init } from "ic10emu_wasm";
// import { Card } from 'bootstrap';

class VirtualMachine {

    constructor() {
        const vm = init();

        window.VM = this;

        this.ic10vm = vm;
        this.ui = new VirtualMachineUI(this);

        this.ics = {}
        const ics = this.ic10vm.ics;
        for (const id of Object.keys(ics)) {
            this.ics[id] = this.ic10vm.getDevice(parseInt(id));
        }
        this.updateCode()


    }

    get devices () {
        return this.ic10vm.devices;
    }

    updateCode() {
        const progs = window.App.session.programs;
        for (const id of Object.keys(progs)) {
            const ic = this.ics[id];
            const prog = progs[id];
            if (ic && prog) {
                console.time(`CompileProgram_${id}`);
                try {
                    this.ics[id].setCode(progs[id]);
                } catch (e) {
                    console.log(e);
                }                
                console.timeEnd(`CompileProgram_${id}`);
            }
        }
        this.update();
    }

    step() {
        const ic = this.ics[window.App.session.activeSession];
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
        const ic = this.ics[window.App.session.activeSession];
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
        const ic = this.ics[window.App.session.activeSession];
        if (ic) {
            ic.reset();
            this.update();
        }
    }

    update() {
        const ic = this.ics[window.App.session.activeSession];
        window.App.session.setActiveLine(window.App.session.activeSession, ic.ip);
        this.ui.update(ic);
    }

    setRegister(index, val) {
        const ic = this.ics[window.App.session.activeSession];
        try {
            ic.setRegister(index, val);
        } catch (e) {
            console.log(e);
        }
    }
}


class VirtualMachineUI {
    constructor(vm) {
        this.vm = vm
        this.state = new VMStateUI(this);
        this.registers = new VMRegistersUI(this);
        this.buildStackDisplay();

        const self = this;


        document.getElementById("vmControlRun").addEventListener('click', (_event) => {
            self.vm.run();
        }, { capture: true });
        document.getElementById("vmControlStep").addEventListener('click', (_event) => {
            self.vm.step();
        }, { capture: true });
        document.getElementById("vmControlReset").addEventListener('click', (_event) => {
            self.vm.reset();
        }, { capture: true });

    }

    update(ic) {
        this.state.update(ic);
        this.registers.update(ic);
    }


    buildStackDisplay() {

    }
}

class VMStateUI {
    constructor(ui) {
        this.ui = ui;
        const stateDom = document.getElementById("vmActiveICState");

        this.tbl = document.createElement("table");
        this.tbl.classList.add("table");
        this.ipRow = this.tbl.insertRow();
        this.counterRow = this.tbl.insertRow()
        this.stateRow = this.tbl.insertRow()
        const ipTh = document.createElement("th");
        ipTh.appendChild(document.createTextNode("Instruction Pointer"));
        this.ipRow.appendChild(ipTh);
        this.instructionPointer = this.ipRow.insertCell();
        const conuterTh = document.createElement("th");
        conuterTh.appendChild(document.createTextNode("Last Run Operations"));
        this.counterRow.appendChild(conuterTh);
        this.instructionCounter = this.counterRow.insertCell();
        const stateTh = document.createElement("th");
        stateTh.appendChild(document.createTextNode("Last State"));
        this.stateRow.appendChild(stateTh);
        this.lastState = this.stateRow.insertCell();

        stateDom.appendChild(this.tbl);
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
        this.tbl.classList.add("d-flex", "flex-wrap", "justify-content-start", "align-items-start", "align-self-center");
        this.regCels = [];
        for (var i = 0; i < 18; i++) {
            const container = document.createElement("div");
            container.classList.add("vm_reg_cel");
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
            aliasesLabel.classList.add("input-group-text")
            aliasesLabel.innerText = "\xa0";
            cell.appendChild(aliasesLabel);
            this.regCels.push({
                cell,
                nameLabel,
                aliasesLabel,
                input,
            });
            container.appendChild(cell);
            this.tbl.appendChild(container);
        }
        this.regCels.forEach(cell => {
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
        const self = this;
        if (ic) {
            const registers = ic.registers;
            if (registers) {
                for (var i = 0; i < registers.length; i++) {
                    this.regCels[i].input.value = registers[i];
                }
            }
            const aliases = ic.aliases;
            if (aliases) {
                this.ic_aliases = {}
                for (const alias of aliases.keys()) {
                    const target = aliases.get(alias);
                    if (target.RegisterSpec &&  target.RegisterSpec.indirection == 0) {
                        const index = target.RegisterSpec.target;
                        this.ic_aliases[alias] = index;
                    }
                }
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
            this.regCels[index].aliasesLabel.innerText = label.join(", ")
        }
    }
}

export { VirtualMachine }