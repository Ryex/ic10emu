from collections import defaultdict
import json
import xml.etree.ElementTree as ET
import argparse
from pathlib import Path
import sys
import re
from itertools import chain
import struct
import binascii

def intOrNone(val):
    try:
        return int(val)
    except ValueError:
        return None

def main():
    arg_parser = argparse.ArgumentParser(
            description="Generate instructions, enums, and docs for lsp.\n\nWorks best when using https://github.com/Ryex/StationeersStationpediaExtractor",
            epilog="Point at the Stationeers install and go!",
            formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    arg_parser.add_argument("path", help="Path to Stationeers installation")
    arg_parser.add_argument("--lang", help="language to extract from (ie. english)", default="english")
    args = arg_parser.parse_args()
    install_path = Path(args.path)
    if install_path.match("Stationeers/*.exe") or install_path.match("Stationeers/rocketstation_Data"):
        install_path = install_path.parent
    elif install_path.name == "Stationeers":
        pass
    elif (install_path / "Stationeers").is_dir():
        install_path = install_path / "Stationeers"
    
    data_path = install_path / "rocketstation_Data" / "StreamingAssets" / "Language" 
    if not data_path.is_dir():

        print(f"Invalid install path. {install_path} does not point to a valid Stationeers installation")
        arg_parser.print_help()
        sys.exit(1)

    lang = args.lang
    if not (data_path / f"{lang}.xml").is_file():
        print("Language file '{lang}.xml' does not exist. can not pull help strings.")
        sys.exit(2)

    extract_data(install_path, data_path, lang)

def extract_data(install_path, data_path: Path, language: str):
    tree = ET.parse(data_path / f"{language}.xml")
    root = tree.getroot()
    interface = root.find("Interface")
    strings = root.find("GameStrings")
    colors = root.find("Colors")
    elms = [elm for elm in (interface, strings, colors) if elm is not None ]

    
    logic_type = re.compile(r"LogicType(\w+)")
    logic_slot_type = re.compile(r"LogicSlotType(\w+)")
    script_command = re.compile(r"ScriptCommand(\w+)")
    script_desc = re.compile(r"ScriptDescription(\w+)")
    color = re.compile(r"Color(\w+)")
    operation_help_strings: dict[str, str] = {}
    enum_help_strings: dict[str, str] = {}
    logictypes: dict[str, tuple[int|None, str]] = {}
    slotlogictypes: dict[str, tuple[int|None, str]] = {}
    for record in chain.from_iterable(elms):
        key = record.find("Key")
        value = record.find("Value")
        if key is None or value is None:
            continue
        key = key.text
        value = value.text
        if key is None or value is None:
            continue
        if match := logic_type.match(key):
            enum_help_strings[f"LogicType.{match.group(1)}"] = value
            logictypes[match.group(1)] = (None, value)
        if match := logic_slot_type.match(key):
            enum_help_strings[f"LogicSlotType.{match.group(1)}"] = value
            slotlogictypes[match.group(1)] = (None, value)
        if match := color.match(key):
            enum_help_strings[f"Color.{match.group(1)}"] = value
        if match := script_command.match(key):
            if not match.group(1).lower() == "command":
                operation_help_strings[f"{match.group(1).lower()}"] = value
        if match := script_desc.match(key):
            operation_help_strings[f"{match.group(1).lower()}"] = value

    op_help_patch_path = Path("data") / "instruction_help_patches.json"
    if op_help_patch_path.exists():
        with op_help_patch_path.open(mode="r") as f:
            patches = defaultdict(dict)
            patches.update(json.load(f))
            operation_help_strings.update(patches[language])

    enums = {}
    with (Path("data") / "enums.txt").open("r") as f:
        for line in f.readlines():
            match line.strip().split(' ', maxsplit=2):
                case [name, val]:
                    help = ""
                    if name in enum_help_strings:
                        help = enum_help_strings[name]
                    enums[name] = (intOrNone(val), help)
                case [name, val, help]:
                    enums[name] = (intOrNone(val), help)
                case _:
                    pass

    with (Path("data") / "logictypes.txt").open("r") as f:
        for line in f.readlines():
            match line.strip().split(' ', maxsplit=2):
                case [name, val]:
                    help = ""
                    if f"LogicType.{name}" in enum_help_strings:
                        help = enum_help_strings[f"LogicType.{name}"]
                    elif name in logictypes:
                        help = logictypes[name][1]
                    logictypes[name] = (intOrNone(val), help)
                case [name, val, help]:
                    logictypes[name] = (intOrNone(val), help)
                case _:
                    pass
    
    with(Path("data") / "slotlogictypes.txt").open("r") as f:
        for line in f.readlines():
            match line.strip().split(' ', maxsplit=2):
                case [name, val]:
                    help = ""
                    if f"LogicSlotType.{name}" in enum_help_strings:
                        help = enum_help_strings[f"LogicSlotType.{name}"]
                    elif name in slotlogictypes:
                        help = slotlogictypes[name][1]
                    slotlogictypes[name] = (intOrNone(val), "")
                case [name, val, help]:
                    slotlogictypes[name] = (intOrNone(val), help)
                case _:
                    pass
    
    batchmodes = {}
    with(Path("data") / "batchmodes.txt").open("r") as f:
        for line in f.readlines():
            match line.strip().split(' ', maxsplit=2):
                case [name, val]:
                    batchmodes[name] = (intOrNone(val), "")
                case [name, val, help]:
                    batchmodes[name] = (intOrNone(val), help)
                case _:
                    pass

    reagentmodes = {}
    with(Path("data") / "reagentmodes.txt").open("r") as f:
        for line in f.readlines():
            match line.strip().split(' ', maxsplit=2):
                case [name, val]:
                    reagentmodes[name] = (intOrNone(val), "")
                case [name, val, help]:
                    reagentmodes[name] = (intOrNone(val), help)
                case _:
                    pass

    enum_values_path = install_path / "Stationpedia" / "Enums.json"
    if enum_values_path.exists():
        with enum_values_path.open(mode="r") as f:
            enum_values = json.load(f)
            def update_enum(enum, values):
                for name, val, in values.items():
                    if name in enum:
                        _, help = enum[name]
                        enum[name] = (val, help)
                    else:
                        help = ""
                        if name in enum_help_strings:
                            help = enum_help_strings[name]
                        enum[name] = (val, help)
            update_enum(logictypes, enum_values["LogicType"])
            update_enum(slotlogictypes, enum_values["LogicSlotType"])
            update_enum(batchmodes, enum_values["LogicBatchMethod"])
            update_enum(reagentmodes, enum_values["LogicReagentMode"])
            update_enum(enums, enum_values["Enums"])
 
    op_help_path = Path("data") / "instructions_help.txt"
    with op_help_path.open(mode="w") as f:
        for key, val in sorted(operation_help_strings.items()):
            f.write("{} {}\n".format(key, val.replace("\r", "").replace("\n", "\\n")))

    stationpedia: dict[str, tuple[str, str | None]] = {}
    things = root.find("Things")
    reagents = root.find("Reagents")
    hashables = [elm for elm in (things, reagents) if elm is not None]
    for record in chain.from_iterable(hashables):
        key = record.find("Key")
        value = record.find("Value")
        if key is None or value is None:
            continue
        key = key.text
        value = value.text
        if key is None:
            continue
        crc = binascii.crc32(key.encode('utf-8'))
        crc_s = struct.unpack("i", struct.pack("I", crc))[0]
        stationpedia[crc_s] = (key, value)
    
    exported_stationpedia_path = install_path / "Stationpedia" / "Stationpedia.json"
    if exported_stationpedia_path.exists():
        with exported_stationpedia_path.open(mode="r") as f:
            exported = json.load(f)
            for page in exported["pages"]:
                stationpedia[page["PrefabHash"]] = (page["PrefabName"], page["Title"])
                
    hashables_path = Path("data") / "stationpedia.txt"
    with hashables_path.open(mode="w") as f:
        for key, val in sorted(stationpedia.items(), key=lambda i: i[1][0]):
            name = val[0]
            desc = val[1] if val[1] is not None else ""
            f.write("{} {} {}\n".format(key, name, desc.replace("\r", "").replace("\n", "\\n")))

    logic_types_path = Path("data") / "logictypes.txt"
    with logic_types_path.open(mode="w") as f:
        for t, (v, help) in sorted(logictypes.items()):
            f.write(f"{t} {v} {help.replace("\r", "").replace("\n", "\\n")}\n")
    slot_logic_types_path = Path("data") / "slotlogictypes.txt"
    with slot_logic_types_path.open(mode="w") as f:
        for t, (v, help) in sorted(slotlogictypes.items()):
            f.write(f"{t} {v} {help.replace("\r", "").replace("\n", "\\n")}\n")
    batch_modes_path = Path("data") / "batchmodes.txt"
    with batch_modes_path.open(mode="w") as f:
        for t, (v, help) in sorted(batchmodes.items()):
            f.write(f"{t} {v} {help.replace("\r", "").replace("\n", "\\n")}\n")
    reagent_modes_path = Path("data") / "reagentmodes.txt"
    with reagent_modes_path.open(mode="w") as f:
        for t, (v, help) in sorted(reagentmodes.items()):
            f.write(f"{t} {v} {help.replace("\r", "").replace("\n", "\\n")}\n")
    enums_path = Path("data") / "enums.txt"
    with enums_path.open(mode="w") as f:
        for name, (val, help) in sorted(enums.items()):
            f.write(f"{name} {val} {help.replace("\r", "").replace("\n", "\\n")}\n")

if __name__ == "__main__":
    main()
