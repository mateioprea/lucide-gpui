import fs from "fs";
import path from "path";
const LUCIDE_SOURCE_DIR = "./node_modules/lucide-static/icons";
const LUCIDE_DEST_DIR = "./lucide_icons/";

const lucideIcons = fs.readdirSync(LUCIDE_SOURCE_DIR);
lucideIcons.forEach((icon: string) => {
  const iconPath = path.join(LUCIDE_SOURCE_DIR, icon);
  const destPath = path.join(LUCIDE_DEST_DIR, icon);
  if (!fs.existsSync(LUCIDE_DEST_DIR)) {
    fs.mkdirSync(LUCIDE_DEST_DIR, { recursive: true });
  }
  fs.copyFileSync(iconPath, destPath);
});
