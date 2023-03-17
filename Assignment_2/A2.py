import pyaudio as pd
import numpy as np
import IPython.display as ipd
from scipy import signal
import math
import matplotlib.pyplot as mb

def complexNumber():
    output = []
    # for i in range(1,100):
    # i=2
    # j = 5
    for i in range(1,100):
        array = 2+1j
        complex = array*1j
        output.append(complex)
        # j+=1
    return output

def sinusoid(freq=440, dur=3, srate=48000, amp=1.0, phase=0.0):
    t = np.linspace(0,dur,int(srate*dur))
    data = amp*np.sin(2*np.pi*freq*t+phase)
    
    #part 4
    data = signal.square(2 * np.pi * freq * t+phase, duty=(np.sin(2 * np.pi * t)+1)/2)
    # data = signal.square(2 * np.pi * freq * t+phase)
    # data = signal.sawtooth(2 * np.pi * freq * t+phase)
    # data = signal.sawtooth(2 * np.pi * freq * t+phase, 0.5)
    return data

def readMIDI():
    #reads midi
    srate=48000
    data = sinusoid(freq=490, dur=1, srate=srate)
    ipd.Audio(data, rate=srate)


def main():
    list = complexNumber()
    
    print(list)
    mb.plot(list, color = "red")
    mb.show()
    readMIDI()

main()