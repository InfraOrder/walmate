#!/usr/bin/env python3

import subprocess

import os

with open("list_of_schemes") as f:
    content = f.readlines()
content = [x.strip() for x in content]

os.chdir("./schemes")

for link in content:
    subprocess.run(["git", "clone", link])
