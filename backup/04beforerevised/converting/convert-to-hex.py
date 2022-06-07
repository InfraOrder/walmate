initial_file = "colors_unconverted.scss"
desired_file = "colors.scss"
colors_file = "solarflare.yaml"

with open(initial_file, 'r') as initial_file:
    data=initial_file.read()

print(data)

color_data = []
with open(colors_file, 'r') as color_file:
    for line in color_file:
        color_data.append(line.strip())

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

i = 0
for color in color_data:
    data = data.replace("$base0" + list_of_colors[i], color)
    i += 1

f = open(desired_file, "w+")
f.write(data)
f.close()
