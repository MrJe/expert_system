import os
import sys

error = False

for f in os.listdir('.'):
    if f.endswith(".refout"):
        continue
    if f.endswith(".userout"):
        continue
    os.system("../expert_system.py '{}' > '{}.userout'".format(f, f))
    if os.system("diff '{}.refout' '{}.userout'".format(f, f)) != 0:
        error = True
os.system("rm *.userout")
if error:
    sys.exit(1)
sys.exit(0)
