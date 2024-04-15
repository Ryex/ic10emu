import json
import re
from collections import defaultdict
from pathlib import Path
from pprint import pprint
from typing import Any, NotRequired, TypedDict  # type: ignore[Any]


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
    Device: NotRequired[PediaPageDevice]
    Item: NotRequired[PediaPageItem]


class Pedia(TypedDict):
    pages: list[PediaPage]
    reagents: dict[str, int]

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


class DBPage(TypedDict):
    name: str
    hash: int
    title: str
    desc: str
    slots: list[DBSlot] | None
    logic: dict[str, str] | None
    slotlogic: dict[str, list[int]] | None
    modes: dict[int, str] | None
    conn: dict[int, DBPageConnection] | None
    item: NotRequired[DBPageItem]
    device: NotRequired[DBPageDevice]


def extract_all() -> None:
    db: dict[str, DBPage] = {}
    pedia: Pedia = {"pages": [], "reagents": {}}
    linkPat = re.compile(r"<link=\w+><color=[\w#]+>(.+?)</color></link>")
    with (Path("data") / "Stationpedia.json").open("r") as f:
        pedia = json.load(f)
    for page in pedia["pages"]:
        item: DBPage = defaultdict(list)  # type: ignore[reportAssignmentType]

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
                item["name"] = name
                item["hash"] = name_hash
                item["title"] = title
                item["desc"] = re.sub(linkPat, r"\1", desc)
                match slots:
                    case []:
                        item["slots"] = None
                    case _:
                        item["slots"] = [{}] * len(slots)  # type: ignore[reportAssignmentType]
                        for slot in slots:
                            item["slots"][int(slot["SlotIndex"])] = {
                                "name": slot["SlotName"],
                                "typ": slot["SlotType"],
                            }

                match logic:
                    case []:
                        item["logic"] = None
                    case _:
                        item["logic"] = {}
                        for lat in logic:
                            item["logic"][re.sub(linkPat, r"\1", lat["LogicName"])] = (
                                lat["LogicAccessTypes"].replace(" ", "")
                            )

                match slotlogic:
                    case []:
                        item["slotlogic"] = None
                    case _:
                        item["slotlogic"] = {}
                        for slt in slotlogic:
                            item["slotlogic"][
                                re.sub(linkPat, r"\1", slt["LogicName"])
                            ] = [int(s) for s in slt["LogicAccessTypes"].split(", ")]

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
                        print(f"NON-CONFORMING: ")
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
                        print(f"NON-CONFORMING: ")
                        pprint(item_props)
                        return

            case _:
                print(f"NON-CONFORMING: ")
                pprint(page)
                return

        db[name] = item

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
                key: clean_nones(val) for key, val in value.items() if val is not None  # type: ignore[unknown]
            }
        else:
            return value  # type: ignore[Any]

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
                    "reagent_hashes": pedia["reagents"]
                }
            ),
            f,
            indent=1,
            sort_keys=True,
        )


if __name__ == "__main__":
    # extract_logicable()
    extract_all()
