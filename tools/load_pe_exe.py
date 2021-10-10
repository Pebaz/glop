import pefile

pe = pefile.PE('drive/EFI/BOOT/BOOTX64.efi')

for section in pe.sections:
	print(section, '\n')
