from bs4 import BeautifulSoup
import requests
import json
from datetime import datetime


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
    for obj in json_data:
        if obj['type'] == "endpoint":
            endpoints.append(obj)
    print(f"found {len(endpoints)} end point(s)")
    # group by end points
    group_by_endpoint = {}
    for endpoint in endpoints:
        if not group_by_endpoint.get(endpoint['title']):
            group_by_endpoint[endpoint['title']] = []
        group_by_endpoint[endpoint['title']].append(endpoint)

    print(f"found {len(group_by_endpoint)} unique endpoints with different methods")


if __name__ == "__main__":
    main()
