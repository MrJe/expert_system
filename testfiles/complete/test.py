import os
import sys

error = False

for f in os.listdir('.'):
    if f.endswith(".refout"):
        continue
    if f.endswith(".userout"):
        continue
    if f.endswith(".py"):
        continue
    os.system("../../target/debug/expert_system '{}' > '{}.userout'".format(f, f))
    if os.system("diff '{}.refout' '{}.userout'".format(f, f)) != 0:
        error = True
        print f
os.system("rm *.userout")
if error:
    sys.exit(1)
sys.exit(0)
