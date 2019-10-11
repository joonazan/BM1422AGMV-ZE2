import numpy as np
import matplotlib.pyplot as plt
from scipy import fftpack

points = []
with open('two_sines.txt', 'rb') as f:
    for line in f:
        points.append(list(map(int, line.split())))

x = np.array(points)[:, 0]
sampling_rate = 400

def plot():
    X = fftpack.fft(x)
    freqs = fftpack.fftfreq(len(x)) * sampling_rate

    fig, ax = plt.subplots()

    ax.stem(freqs, np.abs(X))
    plt.show()

def spectrogram():
    from scipy import signal

    freqs, times, Sx = signal.spectrogram(x, fs=sampling_rate, window='hanning', scaling='spectrum')

    plt.pcolormesh(times, freqs, 10 * np.log10(Sx), cmap='viridis')
    plt.ylabel('frequency [Hz]')
    plt.xlabel('time [sec]')
    plt.show()

x = x[:1000]
plot()
#spectrogram()
