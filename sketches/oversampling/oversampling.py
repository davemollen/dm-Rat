import numpy as np
from scipy.signal import minimum_phase, firwin, kaiser_beta, upfirdn
import matplotlib.pyplot as plt

def _pad_h(h, up):
    """Store coefficients in a transposed, flipped arrangement.

    For example, suppose upRate is 3, and the
    input number of coefficients is 10, represented as h[0], ..., h[9].

    Then the internal buffer will look like this::

       h[9], h[6], h[3], h[0],   // flipped phase 0 coefs
       0,    h[7], h[4], h[1],   // flipped phase 1 coefs (zero-padded)
       0,    h[8], h[5], h[2],   // flipped phase 2 coefs (zero-padded)

    """
    h_padlen = len(h) + (-len(h) % up)
    h_full = np.zeros(h_padlen, h.dtype)
    h_full[:len(h)] = h
    h_full = h_full.reshape(-1, up).T[:, ::-1].ravel()
    return h_full

def polyphase_matrix(h, up):
    """
    Convert 1D FIR filter `h` into a polyphase matrix for upsampling by `up`.
    Output is reversed within each phase for convolution-style filtering.
    """
    h = np.asarray(h)
    # Pad to make length divisible by up
    pad_len = -len(h) % up
    if pad_len:
        h = np.concatenate([h, np.zeros(pad_len, dtype=h.dtype)])
    
    # Reshape into phases: each row is a group for one phase
    h_reshaped = h.reshape(-1, up)
    
    # Transpose so columns are phases, then reverse each column (filter flip)
    polyphase = h_reshaped[::-1, :]

    return polyphase

print('polyphase', _pad_h(np.array([0,1,2,3,4,5,6,7]), 2))
print('polyphase', polyphase_matrix(np.array([0,1,2,3,4,5,6,7]), 2))

class Upsampler:
    def __init__(self, coefficients, oversample_factor):
        """
        coefficients: list of 8 lists (or np.arrays), each representing a polyphase filter
        """
        self.oversample_factor = oversample_factor
        self.coefficients = np.array(self._create_polyphase_filter(coefficients))
        self.length = len(self.coefficients)

        if not (self.length & (self.length - 1)) == 0:
            raise ValueError("Coefficient length must be a power of two")

        self.buffer = np.zeros((oversample_factor, self.length), dtype=np.float32)
        self.index = 0
        self.mask = self.length

    def _create_polyphase_filter(self, coefficients):
        """
        Convert 1D FIR filter `h` into a polyphase matrix for upsampling by `up`.
        Output is reversed within each phase for convolution-style filtering.
        """
        h = np.asarray(coefficients)
        # Pad to make length divisible by up
        pad_len = -len(h) % self.oversample_factor
        if pad_len:
            h = np.concatenate([h, np.zeros(pad_len, dtype=h.dtype)])
        
        # Reshape into phases: each row is a group for one phase
        h_reshaped = h.reshape(-1, self.oversample_factor)
        
        # Transpose so columns are phases, then reverse each column (filter flip)
        polyphase = h_reshaped[::-1, :]

        return polyphase

    def write(self, input_sample):
        """
        Writes input_sample (a scalar) into the buffer at the current index
        """
        self.index = (self.index + 1) % self.mask
        self.buffer[:, self.index] = 0.
        self.buffer[0, self.index] = input_sample  # only first polyphase gets input

    def upsample(self, input_sample):
        """
        input_sample: a float32 scalar
        returns: np.array of 8 float32 upsampled values
        """
        self.write(input_sample)
        output = np.zeros(self.oversample_factor, dtype=np.float32)

        for j in range(self.oversample_factor):
            for i in range(self.length):
                buffer_index = (self.index + i) % self.mask
                output[j] += self.buffer[j, buffer_index] * self.coefficients[i, j]

        return output



# Oversampling factor should be a number that's a power of two
oversampling = 8
base_length = 32
base_sample_rate = 48000.0

N = base_length * oversampling - 1
sample_rate = base_sample_rate * oversampling
print('FIR table length: ', N)
print('sample_rate', sample_rate)

# The Nyquist rate of the signal.
nyq_rate = sample_rate / 2.0

# The desired attenuation in the stop band, in dB.
ripple_db = 90.0

# Compute the order and Kaiser parameter for the FIR filter.
beta = kaiser_beta(ripple_db)

# The cutoff frequency of the filter.
cutoff_hz = base_sample_rate * 0.47

# Use firwin with a Kaiser window to create a lowpass FIR filter.
taps = firwin(N, cutoff_hz/nyq_rate, window=('kaiser', beta))

taps_min_phase = minimum_phase(taps, method='homomorphic', n_fft=None)
print('taps_min_phase', len(taps_min_phase))

# Generate sine wave
def generate_sine_wave(freq, sample_rate, duration):
    t = np.arange(0, duration, 1 / sample_rate)
    return np.sin(2 * np.pi * freq * t)

# Parameters
input_signal = generate_sine_wave(25000, base_sample_rate, 0.01)  # 5kHz tone

# Upsample signal
upsampler = Upsampler(taps_min_phase, oversample_factor=oversampling)
output = []
for x in input_signal:
    y = upsampler.upsample(x)
    output.append(y)

# Flatten the result to a 1D array
upsampled_signal1 = np.concatenate(output)
upsampled_signal2 = upfirdn(h=taps_min_phase, x=input_signal, up=oversampling)
print('signallength', len(input_signal))
print('upsampledsignallength', len(upsampled_signal1))
print('upsampledsignallength', len(upsampled_signal2))
print('size', (((len(input_signal) - 1) * oversampling + len(taps_min_phase)) - 1) // 1 + 1)

# FFT helper
def plot_freq_response(signal, sample_rate, title):
    n = len(signal)
    freq = np.fft.rfftfreq(n, d=1/sample_rate)
    fft_magnitude = np.abs(np.fft.rfft(signal)) / n

    # Plot
    plt.figure(num=title, figsize=(10, 5))
    plt.plot(freq, 20 * np.log10(fft_magnitude + 1e-12))  # dB scale
    plt.title(title)
    plt.xlabel("Frequency [Hz]")
    plt.ylabel("Magnitude [dB]")
    plt.grid(True)
    plt.tight_layout()
    plt.show()

# plot_freq_response(input_signal, base_sample_rate, "Input Signal Spectrum")
plot_freq_response(upsampled_signal1, base_sample_rate * 2, "Upsampled1 Signal Spectrum")
plot_freq_response(upsampled_signal2, base_sample_rate * 2, "Upsampled2 Signal Spectrum")
