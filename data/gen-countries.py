#!/usr/bin/env python3

from os import path, chdir
import json

SCRIPT_DIR = path.dirname(path.realpath(__file__))

def read_json(path: str):
    result = {}
    allowed_ch = 'ABCDEFGHJKLMNPRSTUVWXYZ1234567890'
    with open(path, 'r') as file:
        countries = json.load(file)

    for code, title in countries.items():
        first, _, span = code.partition('-')

        if span:
            ch_from, ch_till = span
        else:
            ch_from = 'A'
            ch_till = '0'

        for ch in allowed_ch[allowed_ch.index(ch_from):allowed_ch.index(ch_till) + 1]:
            if first not in result:
                result[first] = {}
            result[first][ch] = title

    return result

def main():
    chdir(SCRIPT_DIR)
    countries = read_json("countries.json")

    country_file = path.join(SCRIPT_DIR, '..', 'src', 'dicts', 'country.rs')
    with open(country_file, 'w') as country_out:
        country_out.write("//Generated file\n\n")
        country_out.write("const UNKNOWN: &str = \"Unknown\";\n\n")

        country_out.write("pub const fn map_wmi_to_country(wmi: &str) -> &'static str {\n")
        country_out.write("    match wmi.as_bytes()[0] {\n")
        for first, countries in countries.items():
            country_out.write("        b'{}' => match wmi.as_bytes()[1] {{\n".format(first))
            for second, title in countries.items():
                print("{}{}:{}".format(first, second, title))
                country_out.write("            b'{}' => \"{}\",\n".format(second, title))
            country_out.write("            _ => UNKNOWN,\n")
            country_out.write('        },\n')

        country_out.write('        _ => UNKNOWN,\n')
        country_out.write('    }\n')
        country_out.write('}\n')

if __name__ == "__main__":
    main()
