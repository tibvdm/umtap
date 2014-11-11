#!/usr/bin/env python

import json
from json import JSONEncoder


class Taxon(JSONEncoder):
    def __init__(self, id, name, rank, parent_id, valid_taxon):
        self.id = id
        self.name = name
        self.rank = rank
        self.parent_id = parent_id
        self.valid_taxon = valid_taxon
        self.children = set()

    def to_json(self):
        """This must be the worst printer I've ever written. But hey, at least it doesn't uses 15 gigs of memory"""
        print('{')
        print(  '"id": "' + str(self.id) + '",')
        print(  '"name": "' + self.name + '",')
        print(  '"data": {')
        print(    '"count": 1,')
        print(    '"self_count": 1,')
        print(    '"valid_taxon": ' + str(self.valid_taxon) + ',')
        print(  '},')
        print(  '"children": [')
        for child in self.children:
            child.to_json()
            print(',')
        print(  ']')
        print('}')


def read_file():
    with open('taxons.tsv') as f:
        # Skip the first two lines (`header` and `0 name 0 1`)
        next(f)
        next(f)

        for line in f:
            yield line.split("\t")


def read_taxons(taxons):
    for line in read_file():
        taxons[int(line[0])] = (Taxon(
            int(line[0]),
            line[1],
            line[2],
            int(line[3]),
            int(line[4])
        ))


def add_taxon_children(taxons):
    for taxon in taxons:
        if taxon and taxon.id != taxon.parent_id:
            taxons[taxon.parent_id].children.add(taxon)


if __name__ == "__main__":
    taxons = [None] * (1500560 + 1) # YES. MAGIC NUMBERS. BITE ME FELIX.

    read_taxons(taxons)
    add_taxon_children(taxons)

    taxons[1].to_json()
