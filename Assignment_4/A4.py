import IPython.display as ipd
import numpy as np
import wave
import bokeh 
import matplotlib as plt
from bokeh.io import output_notebook
from bokeh.plotting import figure, output_file, show
from scipy.io import wavfile
import librosa
import boreal

def hz2radians(f, srate):
    return 2 * np.pi * f / srate

y2,srate = librosa.load("drop.wav")
mag_spectrum = abs(np.fft.rfft(y2))
p = figure(plot_width=600, plot_height=200)
freqs = np.linspace(0, 0.5 * srate, len(mag_spectrum))
max_freq_bin = int(srate / len(mag_spectrum) * 2000)
p.line(freqs[0:max_freq_bin],mag_spectrum[0:max_freq_bin] * 2 * (1.0 / srate))
show(p)



plt.plot("d")
plt.grid()
plt.xlabel('Time (seconds)')
plt.ylabel('Amplitude')