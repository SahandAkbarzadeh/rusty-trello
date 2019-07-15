from dataclasses import dataclass
from typing import List

from bs4 import BeautifulSoup
import requests
import json
from datetime import datetime


@dataclass
class Entity:
    entity_object: object
    endpoints: List[object]


def main():
    # fetch trello api doc page
    print("Getting Trello Api")
    raw_html = requests.get('https://developers.trello.com/reference#introduction').content
    # find json-data
    html = BeautifulSoup(raw_html, "html.parser")
    json_data = html.find(id="docs").get("data-json")
    json_data = json.loads(json_data)
    print(f"found {len(json_data)} object(s)")
    # find endpoints
    endpoints = []
    objects = []
    for obj in json_data:
        if obj['type'] == "endpoint":
            endpoints.append(obj)
        elif obj['type'] == "basic" and obj['title'].endswith('Object'):
            objects.append(obj)
    print(f"found {len(endpoints)} end point(s)")
    # group by end points
    group_by_endpoint = {}
    for endpoint in endpoints:
        if not group_by_endpoint.get(endpoint['title']):
            group_by_endpoint[endpoint['title']] = []
        group_by_endpoint[endpoint['title']].append(endpoint)

    print(f"found {len(group_by_endpoint)} unique endpoints with different methods")

    group_by_category = {}

    for endpoint in endpoints:
        group_name = endpoint['api']['url'].strip('/').split('/')[0]
        if not group_by_category.get(group_name):
            group_by_category[group_name] = []
        group_by_category[group_name].append(endpoint)

    print(f"found {len(group_by_category)} categories")

    [print(f"\t{category}") for category in group_by_category]

    join_category_with_object = []
    for category in group_by_category:
        entity_object = [x for x in objects if x['title'].split(' ')[0].lower() == category[:-1]]
        join_category_with_object.append(
            Entity(
                endpoints=group_by_category[category],
                entity_object=entity_object[0] if len(entity_object) == 1 else None
            )
        )

    print("")


if __name__ == "__main__":
    main()
