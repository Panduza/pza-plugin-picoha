import time
import logging
from panduza import Reactor

"""
"""

# Start logging
logging.basicConfig(level=logging.DEBUG)

# Create Panduza reactor
print("start connection")
r = Reactor()
r.start()
print("connection ok")


led_couple = [ [2, 3], [4, 5], [6, 7], [8, 9], [10, 11], [12, 13], [14, 15], [16, 17], [18, 19], [20, 21] ]

delay_time = 0.25

for couple in led_couple:
    pa = couple[0]
    pb = couple[1]
    print(f"Testing pins {pa} and {pb}")

    pin_a_dir = r.attribute_from_name(f"pin/{pa}/direction")
    pin_b_dir = r.attribute_from_name(f"pin/{pb}/direction")

    pin_a_dir.set("output")
    pin_b_dir.set("output")

    pin_a_val = r.attribute_from_name(f"pin/{pa}/value")
    pin_b_val = r.attribute_from_name(f"pin/{pb}/value")

    for _ in range(2):
        pin_a_val.set("low")
        pin_b_val.set("high")
        time.sleep(delay_time)
        pin_b_val.set("low")
        pin_a_val.set("high")
        time.sleep(delay_time)
