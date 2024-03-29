import json
from pathlib import Path
from pprint import pprint
from collections import defaultdict
import re
import json


def extract_logicable():
    logicable = []
    pedia = {}
    with Path("./Stationpedia.json").open("r") as f:
        pedia.update(json.load(f))
    for page in pedia["pages"]:
        if page["LogicInsert"] or page["LogicSlotInsert"]:
            logicable.append(page)
    # print(f"{len(logicable)} of {len(pedia["pages"])} are logicable")
    return logicable


def extract_all():
    items = {}
    pedia = {}
    linkPat = re.compile(r"<link=\w+><color=[\w#]+>(.+?)</color></link>")
    with Path("./Stationpedia.json").open("r") as f:
        pedia.update(json.load(f))
    for page in pedia["pages"]:
        item = defaultdict(list)
        match page:
            case {
                "Key": _,
                "Title": _,
                "Description": desc,
                "PrefabName": name,
                "PrefabHash": hash,
                "SlotInserts": slots,
                "LogicInsert": logic,
                "LogicSlotInsert": slotlogic,
                "ModeInsert": modes,
                "ConnectionInsert": connections,
            }:
                item["name"] = name
                item["hash"] = hash
                item["desc"] = re.sub(linkPat, r"\1", desc)
                match slots:
                    case []:
                        item["slots"] = None
                    case _:
                        item["slots"] = [{}] * len(slots)
                        for slot in slots:
                            item["slots"][int(slot["SlotIndex"])] = {
                                "name": slot["SlotName"],
                                "type": slot["SlotType"],
                            }

                match logic:
                    case []:
                        item["logic"] = None
                    case _:
                        item["logic"] = {}
                        for lat in logic:
                            item["logic"][re.sub(linkPat, r"\1", lat["LogicName"])] = (
                                lat["LogicAccessTypes"]
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
                    case []:
                        item["conn"] = None
                    case _:
                        item["conn"] = {}
                        for conn in connections:
                            item["conn"][int(conn["LogicAccessTypes"])] = conn[
                                "LogicName"
                            ]

            case _:
                print(f"NON-CONFORMING: ")
                pprint(page)
                return

        items[name] = item

    logicable = [item["name"] for item in items.values() if item["logic"] is not None]
    slotlogicable = [
        item["name"] for item in items.values() if item["slotlogic"] is not None
    ]
    devices = [
        item["name"]
        for item in items.values()
        if item["logic"] is not None and item["conn"] is not None
    ]

    with open("database.json", "w") as f:
        json.encoder
        json.dump(
            {
                "logic_enabeled": logicable,
                "slot_logic_enabeled": slotlogicable,
                "devices": devices,
                "items": items,
            },
            f,
        )


if __name__ == "__main__":
    # extract_logicable()
    extract_all()
