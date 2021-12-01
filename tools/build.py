import sys, os, shutil
from pathlib import Path

sys.argv.pop(0)

if not sys.argv:
    print('Usage:\n  python build.py some-file.asm')
    sys.exit()

asm, = sys.argv

full_path = str(Path(asm).expanduser().absolute().resolve().with_suffix(''))

cwd = os.getcwd()

try:
    os.chdir('C:/soft/uefi-vm-bc-asm')
    os.system(f'make {full_path}')
    os.chdir(cwd)
    artifact = Path(full_path).with_suffix('.efi')
    if artifact.exists():
        shutil.move(artifact, 'drive/EFI/BOOT/BOOTX64.efi')
except Exception as e:
    print(e)
finally:
    os.chdir(cwd)
