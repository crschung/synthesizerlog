import numpy as np
import IPython.display as ipd

import matpoltlib.pyplot as plt

def sinusoid(freq=440, dur=3, srate=48000, amp=1.0, phase=0.0):
    t = np.linspace(0,dur,int(srate*dur))
    data = amp*np.sin(2*np.pi*freq*t+phase)
    return data

srate=48000
data = sinusoid(freq=440, dur=3, srate=srate)
print(data.shape)
ipd.Audio(data, rate=srate)
