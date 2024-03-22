import json
from pathlib import Path
from pprint import pprint

def extract_logicable():
    logicable = []
    pedia = {}
    with Path("./FullStationpedia.json").open("r") as f:
        pedia.update(json.load(f))
    for page in pedia["pages"]:
        if page["LogicInsert"] or page["LogicSlotInsert"]:
            logicable.append(page)
    print(f"{len(logicable)} of {len(pedia["pages"])} are logicable")




if __name__ == "__main__":
    extract_logicable()
