desired_file = "walmate.tmTheme"
colors_file = "colors (temp)"

with open(desired_file, 'r') as myfile:
    data=myfile.read()

color_data = []
with open(colors_file, 'r') as myfile:
    for line in myfile:
        color_data.append(line.strip())

print(color_data)
# print(data)

list_of_colors = [
    "0",
    "0",
    "1",
    "1",
    "2",
    "2",
    "2",
    "3",
    "3",
    "3",
    "4",
    "4",
    "5",
    "5",
    "5",
    "6",
    "6",
    "7",
    "8",
    "A",
    "C",
]

named_color_data = []
i = 0
for color in color_data:
    # named_color_data.append(("$base0" + list_of_colors[i], color))
    data = data.replace(color, "$base0" + list_of_colors[i])
    i += 1

print(data)

f = open(desired_file,"w+")
f.write(data)
f.close()

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

# for i, color in enumerate(list_of_colors):
#     data_to_replace = "{{base0" + color + "-hex}}"
#     print(data_to_replace)
#     data_to_replace_with = "$base0" + color
#     # data_to_replace = "color-" + color
#     # data_to_replace_with = "$" + "base" + colors_to_replace[i]
#     # print(data_to_replace)
#     data = data.replace(data_to_replace, data_to_replace_with)
#
# f = open(desired_file,"w+")
# f.write(data)
# f.close()
