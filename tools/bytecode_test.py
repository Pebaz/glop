
inst = set()

with open('tools/write_bytecode.py') as file:
    for line in file:
        if '#' in line:
            instruction = line.split('#')[1].strip()

            print(instruction)

            inst.add(instruction.split()[0].strip().replace('32', '').replace('64', ''))

print(len(inst))
print((inst))
