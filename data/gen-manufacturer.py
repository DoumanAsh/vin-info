#!/usr/bin/env python3

from os import path, chdir
from glob import iglob
import json

SCRIPT_DIR = path.dirname(path.realpath(__file__))

def determine_region(wmi):
    region_map = {
        "A": "Africa",
        "B": "Africa",
        "C": "Africa",
        "D": "Africa",
        "E": "Africa",
        "F": "Africa",
        "G": "Africa",
        "H": "Africa",

        "J": "Asia",
        "K": "Asia",
        "L": "Asia",
        "M": "Asia",
        "N": "Asia",
        "P": "Asia",
        "R": "Asia",

        "S": "Europe",
        "T": "Europe",
        "U": "Europe",
        "V": "Europe",
        "W": "Europe",
        "X": "Europe",
        "Y": "Europe",
        "Z": "Europe",

        "1": "NorthAmerica",
        "2": "NorthAmerica",
        "3": "NorthAmerica",
        "4": "NorthAmerica",
        "5": "NorthAmerica",

        "6": "Oceania",
        "7": "Oceania",

        "8": "SouthAmerica",
        "9": "SouthAmerica",
    }

    return region_map.get(wmi[0], None)

def main():
    chdir(SCRIPT_DIR)
    manufacturers = []

    for file in iglob("WMI_*.json"):
        print(">{}...".format(file))
        with open(file, 'r') as file:
            data = json.load(file)
            print("Count: {}".format(data['Count']))
            results = data['Results']

            if not isinstance(results, list):
                print("'Results' is not list of objects")
                continue

            for idx, item in enumerate(results):
                if not isinstance(item, dict):
                    print("'Results' contains non-object at idx={}".format(idx))

                # Filter out bullshit data from API
                country = item['Country']
                if country is None:
                    continue
                country = country.title()

                wmi = item['WMI']
                if len(wmi) != 3:
                    #print("WMI={} with {} chars only".format(wmi, len(wmi)))
                    continue

                region = determine_region(wmi)
                if region is None:
                    print("WMI={} has invalid region".format(wmi))
                    continue

                name = item['Name']
                if name is None:
                    continue
                name = name.title()

                manufacturers.append([wmi, country, name])

    manufacturer_file = path.join(SCRIPT_DIR, '..', 'src', 'dicts', 'manufacturer.rs')
    with open(manufacturer_file, 'w') as manufacturer_out:
        manufacturer_out.write("//Generated file\n\n")
        manufacturer_out.write("const UNKNOWN: &str = \"Unknown\";\n\n")
        manufacturer_out.write("pub const fn map_wmi_to_manufacturer(wmi: &str) -> &'static str {\n")

        man_mapping = {}
        for m_code, _, m_name in manufacturers:
            print("{}:{}".format(m_code, m_name))
            first = m_code[0]
            second = m_code[1]
            third = m_code[2]
            if first not in man_mapping:
                man_mapping[first] = {}
            if second not in man_mapping[first]:
                man_mapping[first][second] = []

            man_mapping[first][second].append([third, m_name])

        manufacturer_out.write("    match wmi.as_bytes()[0] {\n")
        for first, seconds in man_mapping.items():
            manufacturer_out.write("        b'{}' => match wmi.as_bytes()[1] {{\n".format(first))
            for second, thirds in seconds.items():
                manufacturer_out.write("            b'{}' => match wmi.as_bytes()[2] {{\n".format(second))
                for third, m_name in thirds:
                    manufacturer_out.write("                b'{}' => \"{}\",\n".format(third, m_name))
                manufacturer_out.write("                _ => UNKNOWN,\n")
                manufacturer_out.write('            },\n')
            manufacturer_out.write("            _ => UNKNOWN,\n")
            manufacturer_out.write('        },\n')

        manufacturer_out.write('        _ => UNKNOWN,\n')
        manufacturer_out.write('    }\n')
        manufacturer_out.write('}\n')

if __name__ == "__main__":
    main()
