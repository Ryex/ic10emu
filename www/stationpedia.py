import json
from pathlib import Path
from pprint import pprint
from collections import defaultdict
import re
import json


def extract_all():
    items = {}
    pedia = {}
    linkPat = re.compile(r"<link=\w+><color=[\w#]+>(.+?)</color></link>")
    with (Path("data") / "Stationpedia.json").open("r") as f:
        pedia.update(json.load(f))
    for page in pedia["pages"]:
        item = defaultdict(list)
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
                "ConnectionList": connections,  # type: List[Tuple[str, str]]
            }:
                item["name"] = name
                item["hash"] = name_hash
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
                    case []:
                        item["conn"] = None
                    case _:
                        item["conn"] = {}
                        for index, [conn_typ, conn_role] in enumerate(connections):
                            item["conn"][index] = [conn_typ, conn_role]

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

    def clean_nones(value):
        if isinstance(value, list):
            return [clean_nones(x) for x in value if x is not None]
        elif isinstance(value, dict):
            return {
                key: clean_nones(val) for key, val in value.items() if val is not None
            }
        else:
            return value

    with open("data/database.json", "w") as f:
        json.encoder
        json.dump(
            clean_nones(
                {
                    "logic_enabled": logicable,
                    "slot_logic_enabled": slotlogicable,
                    "devices": devices,
                    "items": items,
                }
            ),
            f,
        )


if __name__ == "__main__":
    # extract_logicable()
    extract_all()
