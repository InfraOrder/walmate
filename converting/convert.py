#!/bin/env python3
desired_file = "walmate.kak"

with open(desired_file, 'r') as myfile:
    data=myfile.read()

print(data)

list_of_colors = [
    "0", # "bg",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7", #"fg",
    "8",
    "9",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
]

# colors_to_replace = [
    # "00",
    # "08",
    # "09",
    # "0A",
    # "0B",
    # "0C",
    # "0D",
    # "07",
    # "0E"
# ]

for i, color in enumerate(list_of_colors):
    data_to_replace = "{{base0" + color + "-hex}}"
    print(data_to_replace)
    data_to_replace_with = "$base0" + color
    # data_to_replace = "color-" + color
    # data_to_replace_with = "$" + "base" + colors_to_replace[i]
    # print(data_to_replace)
    data = data.replace(data_to_replace, data_to_replace_with)

f = open(desired_file,"w+")
f.write(data)
f.close()
