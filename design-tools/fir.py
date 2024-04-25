#!python

from numpy import pi, absolute
from scipy.signal import minimum_phase, firwin, freqz, kaiser_beta
from pylab import figure, clf, plot, xlabel, ylabel, ylim, title, grid, show

#------------------------------------------------
# Create a FIR filter and apply it to x.
#------------------------------------------------

# Oversampling factor should be a number that's a power of two
oversampling = 16
base_length = 32
base_sample_rate = 44100.0

N = base_length * oversampling - 1
sample_rate = base_sample_rate * oversampling
print('FIR table length: ', N)
print('sample_rate', sample_rate)

# The Nyquist rate of the signal.
nyq_rate = sample_rate / 2.0

# The desired width of the transition from pass to stop,
# relative to the Nyquist rate.  We'll design the filter
# with a 5 Hz transition width.
width = 2050.0/nyq_rate
# width = 5.0/nyq_rate

# The desired attenuation in the stop band, in dB.
ripple_db = 90.0

# Compute the order and Kaiser parameter for the FIR filter.
# N, beta = kaiserord(ripple_db, width)
beta = kaiser_beta(ripple_db)

# The cutoff frequency of the filter.
cutoff_hz = 22050.0 - 2050.0
# cutoff_hz = 5300.0

# Use firwin with a Kaiser window to create a lowpass FIR filter.
taps = firwin(N, cutoff_hz/nyq_rate, window=('kaiser', beta))

taps_min_phase = minimum_phase(taps, method='homomorphic', n_fft=None)
print('FIR coefficients: ', taps_min_phase)

#------------------------------------------------
# Plot the minimum phase FIR filter coefficients.
#------------------------------------------------

figure(1)
plot(taps_min_phase, 'bo-', linewidth=2)
title('Filter Coefficients (%d taps)' % len(taps_min_phase))
grid(True)

#------------------------------------------------
# Plot the magnitude response of the filter.
#------------------------------------------------

figure(2)
clf()
w, h = freqz(taps, worN=4096)
plot((w/pi)*nyq_rate, absolute(h), linewidth=2)
xlabel('Frequency (Hz)')
ylabel('Gain')
title('Frequency Response')
ylim(-0.05, 1.05)
grid(True)

show()