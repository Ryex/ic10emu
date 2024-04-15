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
    ConnectionList: NotRequired[list[list[str]]]
    SlotClass: NotRequired[str]
    SortingClass: NotRequired[str]
    DevicesLength: NotRequired[int]
    HasReagents: NotRequired[bool]
    HasAtmosphere: NotRequired[bool]
    HasLockState: NotRequired[bool]
    HasOpenState: NotRequired[bool]
    HasOnOffState: NotRequired[bool]
    HasActivateState: NotRequired[bool]
    HasModeState: NotRequired[bool]
    HasColorState: NotRequired[bool]
    IsDynamic: NotRequired[bool]
    IsDevice: NotRequired[bool]
    FilterType: NotRequired[str]
    MaxQuantity: NotRequired[float]


class Pedia(TypedDict):
    pages: list[PediaPage]


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


class DBPage(TypedDict):
    name: str
    hash: int
    desc: str
    slots: list[DBSlot] | None
    logic: dict[str, str] | None
    slotlogic: dict[str, list[int]] | None
    modes: dict[int, str] | None
    conn: dict[int, list[str]] | None
    slotclass: str | None
    sorting: str | None
    pins: int | None
    dynamic: bool
    device: bool
    reagents: bool
    atmosphere: bool
    states: NotRequired[DBPageStates]
    filtertype: NotRequired[str]
    maxquantity: NotRequired[int]


def extract_all() -> None:
    db: dict[str, DBPage] = {}
    pedia: Pedia = {"pages": []}
    linkPat = re.compile(r"<link=\w+><color=[\w#]+>(.+?)</color></link>")
    with (Path("data") / "Stationpedia.json").open("r") as f:
        pedia = json.load(f)
    for page in pedia["pages"]:
        item: DBPage = defaultdict(list)  # type: ignore[reportAssignmentType]

        match page:
            case {
                "Key": _,
                "Title": _,
                "Description": desc,
                "PrefabName": name,
                "PrefabHash": name_hash,
                "SlotInserts": slots,
                "LogicInsert": logic,
                "LogicSlotInsert": slotlogic,
                "ModeInsert": modes,
                "ConnectionInsert": _,
            }:
                connections = page.get("ConnectionList", None)
                slotclass = page.get("SlotClass", None)
                sortingclass = page.get("SortingClass", None)
                deviceslength = page.get("DevicesLength", None)
                hasRreagents = page.get("HasReagents", None)
                hasAtmosphere = page.get("HasAtmosphere", None)
                hasLockState = page.get("HasLockState", None)
                hasOpenState = page.get("HasOpenState", None)
                hasOnOffState = page.get("HasOnOffState", None)
                hasActivateState = page.get("HasActivateState", None)
                hasModeState = page.get("HasModeState", None)
                hasColorState = page.get("HasColorState", None)
                isDynamic = page.get("IsDynamic", None)
                isDevice = page.get("IsDevice", None)
                filterType = page.get("FilterType", None)
                maxQuantity = page.get("MaxQuantity", None)

                item["name"] = name
                item["hash"] = name_hash
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

                match connections:
                    case [] | None:
                        item["conn"] = None
                    case _:
                        item["conn"] = {}
                        for index, [conn_typ, conn_role] in enumerate(connections):
                            item["conn"][index] = [conn_typ, conn_role]

                match hasRreagents:
                    case None:
                        item["reagents"] = False
                    case _:
                        item["reagents"] = hasRreagents

                match hasAtmosphere:
                    case None:
                        item["atmosphere"] = False
                    case _:
                        item["atmosphere"] = hasAtmosphere

                states: DBPageStates = {}

                match hasLockState:
                    case None:
                        pass
                    case _:
                        states["lock"] = hasLockState

                match hasOpenState:
                    case None:
                        pass
                    case _:
                        states["open"] = hasOpenState

                match hasModeState:
                    case None:
                        pass
                    case _:
                        states["mode"] = hasModeState

                match hasActivateState:
                    case None:
                        pass
                    case _:
                        states["activate"] = hasActivateState

                match hasOnOffState:
                    case None:
                        pass
                    case _:
                        states["onoff"] = hasOnOffState

                match hasColorState:
                    case None:
                        pass
                    case _:
                        states["color"] = hasColorState

                if len(list(states.keys())) > 0:
                    item["states"] = states
                item["slotclass"] = slotclass
                item["sorting"] = sortingclass
                item["pins"] = deviceslength
                item["dynamic"] = isDynamic is True
                item["device"] = isDevice is True

                match filterType:
                    case None:
                        pass
                    case _:
                        item["filtertype"] = filterType

                match maxQuantity:
                    case None:
                        pass
                    case _:
                        item["maxquantity"] = int(maxQuantity)

            case _:
                print(f"NON-CONFORMING: ")
                pprint(page)
                return

        db[name] = item

    logicable = [item["name"] for item in db.values() if item["logic"] is not None]
    slotlogicable = [
        item["name"] for item in db.values() if item["slotlogic"] is not None
    ]

    devices = [item["name"] for item in db.values() if item["device"] is True]

    structures = [
        item["name"] for item in db.values() if item["name"].startswith("Structure")
    ]

    items = [item["name"] for item in db.values() if item["dynamic"] is True]

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
                }
            ),
            f,
            indent=1,
            sort_keys=True,
        )


if __name__ == "__main__":
    # extract_logicable()
    extract_all()
