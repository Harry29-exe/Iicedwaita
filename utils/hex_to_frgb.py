def hex_to_rgb(hex_code):
    hex_code = hex_code.strip('#')  # Remove '#' if present
    r = int(hex_code[0:2], 16)  # Convert first two characters to decimal (red component)
    g = int(hex_code[2:4], 16)  # Convert next two characters to decimal (green component)
    b = int(hex_code[4:6], 16)  # Convert last two characters to decimal (blue component)
    return r, g, b

def rgb_to_normalized(rgb):
    r, g, b = rgb
    normalized_r = r / 255.0
    normalized_g = g / 255.0
    normalized_b = b / 255.0
    return normalized_r, normalized_g, normalized_b

# Example usage
hex_codes = [
    "#1b8553",
    "#9c6e03",
    "#fafafa",
    "#ffffff",
    "#ebebeb",
    "#fafafa",
    "#ffffff",
    "#fafafa",
    "#ffffff",
    "#ffffff",
]

for hex_code in hex_codes:
    rgb = hex_to_rgb(hex_code)
    normalized_rgb = rgb_to_normalized(rgb)
    print(normalized_rgb)
