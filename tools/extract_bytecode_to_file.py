"""
Reads the bootable EFI Bytecode file on the drive and writes it's bytecode to
`bc.bin`.

Possible Extensions:
 - Write each section's information to appropriately named bin files: text.bin
"""

import sys, pefile

DEBUG = False
bytes_ = open('drive/EFI/BOOT/BOOTX64.efi', 'rb').read()
pe = pefile.PE('drive/EFI/BOOT/BOOTX64.efi')

for section in pe.sections:
	name = section.Name.decode().replace('\x00', '')

	if DEBUG:
		print(name, '\n', section, '\n')

	if name == '.text':
		num_bytecode_bytes = section.SizeOfRawData
		bytecode_at = section.PointerToRawData

		if DEBUG:
			print('Data Len:', num_bytecode_bytes)
			print('Data At:', bytecode_at)

		# sys.stdout.buffer.write(
		# 	bytes_[bytecode_at:bytecode_at + num_bytecode_bytes]
		# )

		bits = bytes_[bytecode_at:bytecode_at + num_bytecode_bytes]

		with open('bc.bin', 'wb') as bytecode_file:
			bytecode_file.write(bits)
			break
