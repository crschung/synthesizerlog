# import pyaudio as pd
import math
import matplotlib.pyplot as mb

def complexNumber():
    array = ()
    j = 0
    for i in range(1,100):
        array = array + (2+j,j)
        j+=1
    return array

def readMIDI():
    #reads midi
    return 0


def main():
    list = complexNumber()
    
    print(list)
    mb.plot(list, color = "red")
    mb.show()
    print(readMIDI())

main()