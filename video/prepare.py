# Copies the character frames (512px) and the pixel pouch logo into public/ for Remotion.
import os
from PIL import Image
here=os.path.dirname(os.path.abspath(__file__));root=os.path.dirname(here)
for who in ["claude","deepseek","pouch","claude-m","deepseek-m","pouch-m"]:
    src=os.path.join(root,"assets/characters",who);dst=os.path.join(here,"public/c",who);os.makedirs(dst,exist_ok=True)
    for f in os.listdir(src):
        if f.endswith(".png") and f not in ("icon.png","bust.png","ref.png"):
            Image.open(os.path.join(src,f)).resize((512,512),Image.LANCZOS).save(os.path.join(dst,f))
Image.open(os.path.join(root,"site/icon.png")).save(os.path.join(here,"public/logo.png"))
