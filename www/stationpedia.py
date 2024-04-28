import json
import re
from collections import defaultdict
from pathlib import Path
from pprint import pprint
from typing import Any, NotRequired, TypedDict  # type: ignore[Any]

try:
    import markdown
except ImportError:
    markdown = None


class SlotInsert(TypedDict):
    SlotIndex: str
    SlotName: str
    SlotType: str


class LInsert(TypedDict):
    LogicName: str
    LogicAccessTypes: str


class PediaPageItem(TypedDict):
    SlotClass: str
    SortingClass: str
    MaxQuantity: NotRequired[float]
    FilterType: NotRequired[str]
    Consumable: NotRequired[bool]
    Ingredient: NotRequired[bool]
    Reagents: NotRequired[dict[str, float]]


class PediaPageDevice(TypedDict):
    ConnectionList: list[list[str]]
    HasReagents: bool
    HasAtmosphere: bool
    HasLockState: bool
    HasOpenState: bool
    HasOnOffState: bool
    HasActivateState: bool
    HasModeState: bool
    HasColorState: bool
    DevicesLength: NotRequired[int]


class MemoryInstruction(TypedDict):
    Type: str
    Value: int
    Description: str


class PediaPageMemory(TypedDict):
    MemorySize: int
    MemorySizeReadable: str
    MemoryAccess: str
    Instructions: dict[str, MemoryInstruction] | None


class PediaPageLogicInfo(TypedDict):
    LogicSlotTypes: dict[str, dict[str, str]]
    LogicTypes: dict[str, str]


class PediaPage(TypedDict):
    Key: str
    Title: str
    Description: str
    PrefabName: str
    PrefabHash: int
    SlotInserts: list[SlotInsert]
    LogicInsert: list[LInsert]
    LogicSlotInsert: list[LInsert]
    ModeInsert: list[LInsert]
    ConnectionInsert: list[LInsert]
    LogicInfo: PediaPageLogicInfo | None
    Item: NotRequired[PediaPageItem]
    Device: NotRequired[PediaPageDevice]
    WirelessLogic: bool | None
    Memory: PediaPageMemory | None
    TransmissionReceiver: bool | None


class ScriptCommand(TypedDict):
    desc: str
    example: str


class PediaReagent(TypedDict):
    Hash: int
    Unit: str
    Sources: dict[str, float] | None


class Pedia(TypedDict):
    pages: list[PediaPage]
    reagents: dict[str, int]
    scriptCommands: dict[str, ScriptCommand]


class DBSlot(TypedDict):
    name: str
    typ: str


class DBPageStates(TypedDict):
    lock: NotRequired[bool]
    open: NotRequired[bool]
    mode: NotRequired[bool]
    onoff: NotRequired[bool]
    color: NotRequired[bool]
    activate: NotRequired[bool]


class DBPageConnection(TypedDict):
    typ: str
    role: str
    name: str


class DBPageDevice(TypedDict):
    states: DBPageStates
    reagents: bool
    atmosphere: bool
    pins: NotRequired[int]


class DBPageItem(TypedDict):
    slotclass: str | None
    sorting: str | None
    filtertype: NotRequired[str]
    maxquantity: NotRequired[int]
    consumable: NotRequired[bool]
    ingredient: NotRequired[bool]
    reagents: NotRequired[dict[str, float]]


class DBPageMemoryInstruction(TypedDict):
    typ: str
    value: int
    desc: str


class DBPageMemory(TypedDict):
    size: int
    sizeDisplay: str
    access: str
    instructions: dict[str, DBPageMemoryInstruction] | None


class DBPage(TypedDict):
    name: str
    hash: int
    title: str
    desc: str
    slots: list[DBSlot] | None
    logic: dict[str, str] | None
    slotlogic: dict[str, dict[str, str]] | None
    modes: dict[int, str] | None
    conn: dict[int, DBPageConnection] | None
    item: NotRequired[DBPageItem]
    device: NotRequired[DBPageDevice]
    transmitter: bool
    receiver: bool
    memory: DBPageMemory | None


translation_regex = re.compile(r"<N:([A-Z]{2}):(\w+)>")
translation_keys: set[str] = set()
translation_codes: set[str] = set()


def replace_translation(m: re.Match[str]) -> str:
    match m.groups():
        case (code, key):
            translation_keys.add(key)
            translation_codes.add(code)
            return key
        case _ as g:
            print("bad translation match?", g, m.string)
            return m.string


def trans(s: str) -> str:
    return re.sub(translation_regex, replace_translation, s)


color_regex = re.compile(
    r"<color=(#?\w+)>((:?(?!<color=(?:#?\w+)>).)+?)</color>", re.DOTALL
)
link_regex = re.compile(r"<link=(\w+)>(.+?)</link>")


def strip_color(s: str) -> str:
    replacemnt = r"\2"
    last = s
    new = color_regex.sub(replacemnt, last)
    while new != last:
        last = new
        new = color_regex.sub(replacemnt, last)
    return new


def color_to_html(s: str) -> str:
    replacemnt = r"""<div style="color: \1;">\2</div>"""
    last = s
    new = color_regex.sub(replacemnt, last)
    while new != last:
        last = new
        new = color_regex.sub(replacemnt, last)
    return new


def strip_link(s: str) -> str:
    replacemnt = r"\2"
    last = s
    new = link_regex.sub(replacemnt, last)
    while new != last:
        last = new
        new = link_regex.sub(replacemnt, last)
    return new


def extract_all() -> None:
    db: dict[str, DBPage] = {}
    pedia: Pedia = {"pages": [], "reagents": {}, "scriptCommands": {}}
    with (Path("data") / "Stationpedia.json").open("r") as f:
        pedia = json.load(f)
    for page in pedia["pages"]:
        item: DBPage = {
            "name": "",
            "hash": 0,
            "title": "",
            "desc": "",
            "slots": None,
            "logic": None,
            "slotlogic": None,
            "modes": None,
            "conn": None,
            "transmitter": False,
            "receiver": False,
            "memory": None,
        }
        match page:
            case {
                "Key": _,
                "Title": title,
                "Description": desc,
                "PrefabName": name,
                "PrefabHash": name_hash,
                "SlotInserts": slots,
                "LogicInsert": logic,
                "LogicSlotInsert": slotlogic,
                "ModeInsert": modes,
                "ConnectionInsert": conninsert,
            }:
                connNames = {
                    int(insert["LogicAccessTypes"]): insert["LogicName"]
                    for insert in conninsert
                }

                device = page.get("Device", None)
                item_props = page.get("Item", None)
                logicinfo = page.get("LogicInfo", None)
                wireless = page.get("WirelessLogic", False)
                receiver = page.get("TransmissionReceiver", False)
                memory = page.get("Memory", None)
                item["name"] = name
                item["hash"] = name_hash
                item["title"] = trans(title)
                item["desc"] = trans(strip_link(strip_color(desc)))
                match slots:
                    case []:
                        item["slots"] = None
                    case _:
                        item["slots"] = [{}] * len(slots)  # type: ignore[reportAssignmentType]
                        for slot in slots:
                            item["slots"][int(slot["SlotIndex"])] = {
                                "name": trans(slot["SlotName"]),
                                "typ": slot["SlotType"],
                            }

                match logic:
                    case []:
                        item["logic"] = None
                    case _:
                        item["logic"] = {}
                        for lat in logic:
                            item["logic"][strip_link(strip_color(lat["LogicName"]))] = (
                                lat["LogicAccessTypes"].replace(" ", "")
                            )

                match slotlogic:
                    case []:
                        item["slotlogic"] = None
                    case _:
                        item["slotlogic"] = {}
                        for slt in slotlogic:
                            item["slotlogic"][
                                strip_link(strip_color(slt["LogicName"]))
                            ] = {s: "Read" for s in slt["LogicAccessTypes"].split(", ")}

                match modes:
                    case []:
                        item["modes"] = None
                    case _:
                        item["modes"] = {}
                        for mode in modes:
                            item["modes"][int(mode["LogicAccessTypes"])] = mode[
                                "LogicName"
                            ]

                match device:
                    case None:
                        pass
                    case {
                        "ConnectionList": connections,
                        "HasReagents": hasReagents,
                        "HasAtmosphere": hasAtmosphere,
                        "HasLockState": hasLockState,
                        "HasOpenState": hasOpenState,
                        "HasModeState": hasModeState,
                        "HasOnOffState": hasOnOffState,
                        "HasActivateState": hasActivateState,
                        "HasColorState": hasColorState,
                    }:
                        match connections:
                            case []:
                                item["conn"] = None
                            case _:
                                item["conn"] = {}
                                for index, [conn_typ, conn_role] in enumerate(
                                    connections
                                ):
                                    item["conn"][index] = {
                                        "typ": conn_typ,
                                        "role": conn_role,
                                        "name": connNames.get(index, "Connection"),
                                    }

                        states: DBPageStates = {}

                        states["lock"] = hasLockState
                        states["open"] = hasOpenState
                        states["mode"] = hasModeState
                        states["activate"] = hasActivateState
                        states["onoff"] = hasOnOffState
                        states["color"] = hasColorState

                        deviceslength = device.get("DevicesLength", None)
                        dbdevice: DBPageDevice = {
                            "states": states,
                            "reagents": hasReagents,
                            "atmosphere": hasAtmosphere,
                        }

                        match deviceslength:
                            case None:
                                pass
                            case _:
                                dbdevice["pins"] = deviceslength

                        item["device"] = dbdevice

                    case _:
                        print("NON-CONFORMING: ")
                        pprint(device)
                        return

                match item_props:
                    case None:
                        pass
                    case {"SlotClass": slotclass, "SortingClass": sortingclass}:
                        maxquantity = item_props.get("MaxQuantity", None)
                        filtertype = item_props.get("FilterType", None)
                        dbitem: DBPageItem = {
                            "sorting": sortingclass,
                            "slotclass": slotclass,
                        }
                        match maxquantity:
                            case None:
                                pass
                            case _:
                                dbitem["maxquantity"] = int(maxquantity)

                        match filtertype:
                            case None:
                                pass
                            case _:
                                dbitem["filtertype"] = filtertype

                        consumable = item_props.get("Consumable", None)
                        match consumable:
                            case None:
                                pass
                            case _:
                                dbitem["consumable"] = consumable

                        ingredient = item_props.get("Ingredient", None)
                        match ingredient:
                            case None:
                                pass
                            case _:
                                dbitem["ingredient"] = ingredient

                        reagents = item_props.get("Reagents", None)
                        match reagents:
                            case None:
                                pass
                            case _:
                                dbitem["reagents"] = reagents

                        item["item"] = dbitem
                    case _:
                        print("NON-CONFORMING: ")
                        pprint(item_props)
                        return

                match logicinfo:
                    case None:
                        pass
                    case _:
                        for lt, access in logicinfo["LogicTypes"].items():
                            if item["logic"] is None:
                                item["logic"] = {}
                            item["logic"][lt] = access
                        for slot, slotlogicinfo in logicinfo["LogicSlotTypes"].items():
                            if item["slotlogic"] is None:
                                item["slotlogic"] = {}
                            if slot not in item["slotlogic"]:
                                item["slotlogic"][slot] = {}
                            for slt, access in slotlogicinfo.items():
                                item["slotlogic"][slot][slt] = access

                if wireless:
                    item["transmitter"] = True
                if receiver:
                    item["receiver"] = True

                match memory:
                    case None:
                        pass
                    case _:
                        item["memory"] = {
                            "size": memory["MemorySize"],
                            "sizeDisplay": memory["MemorySizeReadable"],
                            "access": memory["MemoryAccess"],
                            "instructions": None,
                        }
                        instructions = memory.get("Instructions", None)
                        match instructions:
                            case None:
                                pass
                            case _:

                                def condense_lines(s: str) -> str:
                                    return "\r\n".join(
                                        [" ".join(line.split()) for line in s.splitlines()]
                                    )

                                item["memory"]["instructions"] = {
                                    inst: {
                                        "typ": info["Type"],
                                        "value": info["Value"],
                                        "desc": condense_lines(
                                            strip_color(strip_link(info["Description"]))
                                        ),
                                    }
                                    for inst, info in instructions.items()
                                }

            case _:
                print("NON-CONFORMING: ")
                pprint(page)
                return

        db[name] = item

    print("Translation codes:")
    pprint(translation_codes)
    print("Translations keys:")
    pprint(translation_keys)

    logicable = [item["name"] for item in db.values() if item["logic"] is not None]
    slotlogicable = [
        item["name"] for item in db.values() if item["slotlogic"] is not None
    ]

    devices = [item["name"] for item in db.values() if "device" in item]

    structures = [
        item["name"] for item in db.values() if item["name"].startswith("Structure")
    ]

    items = [item["name"] for item in db.values() if "item" in item]

    def clean_nones(value: Any) -> Any:  # type: ignore[Any]
        if isinstance(value, list):
            return [clean_nones(x) for x in value if x is not None]  # type: ignore[unknown]
        elif isinstance(value, dict):
            return {
                key: clean_nones(val)
                for key, val in value.items() # type:ignore[reportUnknownVariable]
                if val is not None
            }
        else:
            return value  # type: ignore[Any]

    enums: dict[str, dict[str, int]] = {}
    with open("data/Enums.json", "r") as f:
        exported_enums: dict[str, dict[str, int]] = json.load(f)
        for cat, cat_enums in exported_enums.items():
            for enum, val in cat_enums.items():
                key = cat
                if cat == "Enums":
                    if "." in enum:
                        key, enum = enum.split(".")
                    else :
                        key = "Condition"
                if key not in enums:
                    enums[key] = {}
                enums[key][enum] = val

    with open("data/database.json", "w") as f:
        json.dump(
            clean_nones(
                {
                    "logic_enabled": logicable,
                    "slot_logic_enabled": slotlogicable,
                    "devices": devices,
                    "structures": structures,
                    "items": items,
                    "db": db,
                    "names_by_hash": {
                        page["hash"]: page["name"] for page in db.values()
                    },
                    "reagents": pedia["reagents"],
                    "enums": enums,
                }
            ),
            f,
            indent=1,
            sort_keys=True,
        )


if __name__ == "__main__":
    # extract_logicable()
    extract_all()
