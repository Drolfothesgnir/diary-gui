import sharp from "sharp";
import fs from "fs/promises";
import path from "path";
import toIco from "png-to-ico";

async function generateIcons(svgPath) {
  // Create icons directory if it doesn't exist
  await fs.mkdir("icons", { recursive: true });

  const sizes = {
    "32x32": { width: 32, height: 32 },
    "128x128": { width: 128, height: 128 },
    "256x256": { width: 256, height: 256 }, // This will be 128x128@2x
    "1024x1024": { width: 1024, height: 1024 }, // For macOS icns
  };

  // Read the SVG file
  const inputBuffer = await fs.readFile(svgPath);

  for (const [name, dimensions] of Object.entries(sizes)) {
    const { width, height } = dimensions;

    // Generate PNG
    await sharp(inputBuffer)
      .resize(width, height, {
        fit: "contain",
        background: { r: 0, g: 0, b: 0, alpha: 0 },
      })
      .png()
      .toFile(path.join("icons", `${name}.png`));
  }

  // Rename files to match Tauri's expected structure
  await fs.rename("icons/32x32.png", "icons/32x32.png");
  await fs.rename("icons/128x128.png", "icons/128x128.png");
  await fs.rename("icons/256x256.png", "icons/128x128@2x.png");

  // For macOS .icns (you'll need a separate tool or library for actual .icns conversion)
  // For Windows .ico (you'll need a separate tool or library for actual .ico conversion)

  // Generate .ico file
  try {
    const icoBuffer = await toIco(["icons/32x32.png", "icons/128x128.png"]);
    await fs.writeFile("icons/icon.ico", icoBuffer);
  } catch (error) {
    console.error("Error generating .ico file:", error);
  }

  console.log("Icons generated successfully!");
}

// Usage
const svgPath = process.argv[2];
if (!svgPath) {
  console.error("Please provide the path to your SVG file");
  process.exit(1);
}

generateIcons(svgPath).catch(console.error);
