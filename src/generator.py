import json
import numpy as np
from PIL import Image, ImageDraw, ImageFont
import os
import sys

# Load sky colours, name, and timezone from generation.json
with open("./config/generation.json", "r") as file:
    config = json.load(file)
    sky_colours = config["sky_colours"]
    name = config["name"]
    timezone = config["timezone"]

# Function to interpolate RGB values between given key times
def interpolate_colour(hour):
    hours = sorted(map(int, sky_colours.keys()))
    for i in range(len(hours) - 1):
        if hours[i] <= hour <= hours[i + 1]:
            t = (hour - hours[i]) / (hours[i + 1] - hours[i])
            c1, c2 = np.array(sky_colours[str(hours[i])]), np.array(
                sky_colours[str(hours[i + 1])]
            )
            return tuple((1 - t) * c1 + t * c2)
    return sky_colours[str(hour)]  # Exact match case

# Generate 24 images if they do not already exist
width, height = 1500, 500
font_size = 50
font_path = "./config/fonts/madecarvingsoft.woff2"  # Adjust if a different font is needed

# Folder to store images
output_folder = "./src/blobs"
print(f"Attempting to create folder: {output_folder}")

# Check and create the folder if it doesn't exist
try:
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)
        print(f"Folder created: {output_folder}")
    else:
        print(f"Folder already exists: {output_folder}")
except Exception as e:
    print(f"Error creating folder: {e}")

# Check if images need to be generated
images_to_generate = []
for hour in range(24):
    image_path = f"{output_folder}/{str(hour).zfill(2)}.png"
    print(f"Image path for hour {hour}: {image_path}")
    if not os.path.exists(image_path):
        images_to_generate.append(hour)

# Regenerate images if needed
if images_to_generate:
    print("Regenerating missing images...")

    # Generate images
    for hour in images_to_generate:
        colour = interpolate_colour(hour)
        fade_ratio = 0.3  # Make the gradient fade lower
        gradient_height = int(height * fade_ratio)
        gradient = np.vstack([
            np.full((height - gradient_height, width, 3), colour, dtype=np.uint8),
            np.linspace(colour, (255, 255, 255), gradient_height).astype(np.uint8).reshape(gradient_height, 1, 3).repeat(width, axis=1)
        ])
        
        # Debugging: Check gradient shape and sample pixel value
        print(f"Hour {hour}: Gradient shape - {gradient.shape}")
        print(f"Hour {hour}: Sample pixel value at (0, 0) - {gradient[0, 0]}")

        # Directly create the image from the gradient array
        img = Image.fromarray(gradient)

        # Debugging: Check image mode and size
        print(f"Hour {hour}: Image mode - {img.mode}")
        print(f"Hour {hour}: Image size - {img.size}")

        # Add text
        draw = ImageDraw.Draw(img)
        try:
            font = ImageFont.truetype(font_path, font_size)
        except IOError:
            font = ImageFont.load_default()

        # Calculate text width and height using textbbox
        bbox = draw.textbbox((0, 0), name, font=font)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]

        position = (20, 20)  # Top-left corner
        text_colour = (255 - int(colour[0]), 255 - int(colour[1]), 255 - int(colour[2]))  # Contrasting colour
        draw.text(position, name, fill=text_colour, font=font)

        # Save image to disk
        image_path = f"{output_folder}/{str(hour).zfill(2)}.png"
        try:
            img.save(image_path)
            print(f"Hour {hour}: Image saved successfully.")
        except Exception as e:
            print(f"Error saving image for hour {hour}: {e}")

    print("Missing images regenerated.")

else:
    print("All images already exist. No regeneration needed.")

print("Generator script completed.")