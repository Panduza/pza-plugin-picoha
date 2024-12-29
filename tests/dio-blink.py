import time
import logging
from panduza import Reactor

"""
This test suppose that only 1 KD3005P is on the bench
"""

# Start logging
logging.basicConfig(level=logging.DEBUG)

# Create Panduza reactor
print("start connection")
r = Reactor()
r.start()
print("connection ok")


pin_4_dir = r.attribute_from_name("pin/4/direction")
pin_5_dir = r.attribute_from_name("pin/5/direction")

pin_4_dir.set("output")
pin_5_dir.set("output")

pin_4_val = r.attribute_from_name("pin/4/value")
pin_5_val = r.attribute_from_name("pin/5/value")

pin_4_val.set("low")
pin_5_val.set("high")

# output_enable_control.set(True)
# time.sleep(1)
# output_enable_control.set(False)
# time.sleep(1)

# # 
# step = 1
# if voltage_control.decimals() != 0:
#     step = 1 / (10 ** voltage_control.decimals())


# i = voltage_control.min()
# while i <= voltage_control.max():
#     print(f"set voltage to {i}{voltage_control.unit()}")
#     i += step
#     voltage_control.set(i)


# voltage_control.set(25)
