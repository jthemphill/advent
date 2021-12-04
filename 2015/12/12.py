import json
import sys

def json_sum(doc):
    if type(doc) == int:
        return doc
    if type(doc) == str:
        return 0
    if type(doc) == list:
        return sum([json_sum(x) for x in doc])
    if type(doc) == dict:
        if "red" in doc.values():
            return 0
        return sum([json_sum(v) for v in doc.values()])
    raise Exception(f"Not sure what this is: {doc}")

doc = json.load(sys.stdin)
print(json_sum(doc))

