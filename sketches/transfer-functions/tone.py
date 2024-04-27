from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz
print(sample_rate / 2)

# Change the filter value to the difference in the frequency response
# Keep it between 0 and 1
filter = 1.
filter *= filter

def get_cutoff_frequency(filter):
  resistor = filter * 100000. + 1500.
  capacitor = 3.3e-9
  return min(1 / (np.pi * 2. * resistor * capacitor), sample_rate / 2)

# Get cutoff frequency from filter value
cutoff_freq = get_cutoff_frequency(filter)
print('cutoff freq:', cutoff_freq)

# Apply Butterworth filter
b, a = signal.butter(1, cutoff_freq, 'low', fs=sample_rate)
print("digital filter coefficients:", (list(b), list(a)))

# Convert to cascaded biquad filters
# Only needed for higher order Butterworth filter
# So it's not really needed, since we have a first order Butterworth filter
sos = signal.tf2sos(b, a)

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig = plt.figure()
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([-70, 0])
plt.show()