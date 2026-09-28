from PIL import Image
from pathlib import Path
for p in Path(__file__).parent.glob('*.png'):
    print(p.name, Image.open(p).size)
