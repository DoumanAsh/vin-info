#!/usr/bin/env python3

from os import path, chdir
import requests

SCRIPT_DIR = path.dirname(path.realpath(__file__))

def fetch_json(typ):
    file = "WMI_{}.json".format(typ)
    url = "https://vpic.nhtsa.dot.gov/api/vehicles/GetWMIsForManufacturer/?vehicleType={}&format=json".format(typ)
    response = requests.get(url)
    if not response.status_code == 200:
        raise(Exception("Failed to fetch WIM data(status={})".format(response.status_code)));
    with open(file, 'w') as output:
        output.write(response.text)
    return file

def main():
    chdir(SCRIPT_DIR)
    print("Fetch car...")
    file = fetch_json("car")
    print(">{}: Written".format(file))

    print("Fetch bus...")
    file = fetch_json("bus")
    print(">{}: Written".format(file))

    print("Fetch motorcycle...")
    file = fetch_json("motorcycle")
    print(">{}: Written".format(file))

    print("Fetch truck...")
    file = fetch_json("truck")
    print(">{}: Written".format(file))

    print("Fetch mpv...")
    file = fetch_json("mpv")
    print(">{}: Written".format(file))

if __name__ == "__main__":
    main()
