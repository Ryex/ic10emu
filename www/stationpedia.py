import json
import re
from collections import defaultdict
from pathlib import Path
from pprint import pprint
from typing import Any, NotRequired  # type: ignore[Any]
from typing import TypedDict


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


class Pedia(TypedDict):
    pages: list[PediaPage]


class DBSlot(TypedDict):
    name: str
    typ: str


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

                item["slotclass"] = slotclass
                item["sorting"] = sortingclass
                item["pins"] = deviceslength


            case _:
                print(f"NON-CONFORMING: ")
                pprint(page)
                return

        db[name] = item

    logicable = [item["name"] for item in db.values() if item["logic"] is not None]
    slotlogicable = [
        item["name"] for item in db.values() if item["slotlogic"] is not None
    ]

    devices = [
        item["name"]
        for item in db.values()
        if item["logic"] is not None and item["conn"] is not None
    ]

    structures = [
        item["name"] for item in db.values() if item["name"].startswith("Structure")
    ]

    items = [item["name"] for item in db.values() if item["name"] not in structures]

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
                    "names_by_hash": {page["hash"]: page["name"] for page in db.values()}
                }
            ),
            f,
            indent=1,
            sort_keys=True,
        )

if __name__ == "__main__":
    # extract_logicable()
    extract_all()
