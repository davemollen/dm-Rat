import numpy as np
from scipy.signal import minimum_phase, firwin, kaiser_beta, upfirdn
import matplotlib.pyplot as plt

class Resampler:
    """
    Polyphase FIR-based upfirdn (upsample, filter, downsample).
    Matches scipy.signal.upfirdn behavior (for 1D real signals).
    """
    def __init__(self, coefficients, up=1, down=1):
        self.up = int(up)
        self.down = int(down)
        self.h = np.asarray(coefficients, dtype=np.float64)

        if self.h.ndim != 1 or self.h.size == 0:
            raise ValueError("coefficients must be 1-D with non-zero length")
        if self.up < 1 or self.down < 1:
            raise ValueError("up and down must be >= 1")

        # Pad filter coefficients so length is multiple of up
        pad = -len(self.h) % self.up
        if pad:
            self.h = np.pad(self.h, (0, pad))

        # Polyphase decomposition
        # Shape: (nphases, up)
        self.polyphase = self.h.reshape(-1, self.up)
        self.phase_len = self.polyphase.shape[0]

        # Input delay line
        self.buffer = np.zeros(self.phase_len, dtype=np.float64)

        # Internal time accumulator for rate conversion
        self.t = 0  

    def _shift_in(self, x):
        """Shift input sample into delay line (newest at the end)."""
        self.buffer[:-1] = self.buffer[1:]
        self.buffer[-1] = x

    def _filter_output(self, phase):
        """Compute filter output for given polyphase index."""
        return np.dot(self.polyphase[:, phase], self.buffer[::-1])

    def process(self, x):
        """
        Process full array with upsampling+downsampling.
        Equivalent to scipy.signal.upfirdn(h, x, up, down).
        """
        x = np.asarray(x, dtype=np.float64)
        outputs = []
        self.t = 0  # reset time index

        # Process input samples
        for i, xi in enumerate(x):
            self._shift_in(xi)
            
            # Generate outputs as long as t < up
            while self.t < self.up:
                y = self._filter_output(self.t)
                outputs.append(y)
                self.t += self.down
            # Reduce phase accumulator
            self.t -= self.up

        # Flush filter tail
        for _ in range(len(self.h) - 1):
            self._shift_in(0.0)
            while self.t < self.up:
                y = self._filter_output(self.t)
                outputs.append(y)
                self.t += self.down
            self.t -= self.up

        return np.array(outputs, dtype=np.float32)

# Oversampling factor should be a number that's a power of two
oversampling = 8
base_length = 32
base_sample_rate = 48000.0

N = base_length * oversampling - 1
sample_rate = base_sample_rate * oversampling

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
input_signal = generate_sine_wave(25000, base_sample_rate, 0.001)  # 5kHz tone
print("input_signal", input_signal.tolist())
# input_signal = generate_sine_wave(30000, base_sample_rate, 1.0)  # 5kHz tone

# Upsample signal
upsampler = Resampler(taps_min_phase, up=oversampling)
upsampled_signal1 = upsampler.process(input_signal)
expected_len = ((len(input_signal) - 1) * oversampling + len(taps_min_phase))
upsampled_signal1 = upsampled_signal1[:expected_len]
upsampled_signal2 = upfirdn(h=taps_min_phase, x=input_signal, up=oversampling)

# Downsample signal
downsampler = Resampler(taps_min_phase, 1, 8)
downsampled_signal1 = downsampler.process(input_signal)
downsampled_signal2 = upfirdn(h=taps_min_phase, x=input_signal, down=oversampling, up=1)

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
    plt.ylim(-250, 0)
    plt.grid(True)

plot_freq_response(input_signal, base_sample_rate, "Input Signal Spectrum")
plot_freq_response(upsampled_signal1, base_sample_rate * oversampling, "Upsampled1 Signal Spectrum")
plot_freq_response(upsampled_signal2, base_sample_rate * oversampling, "Upsampled2 Signal Spectrum")
plot_freq_response(downsampled_signal1, base_sample_rate, "Downsampled1 Signal Spectrum")
plot_freq_response(downsampled_signal2, base_sample_rate, "Downsampled2 Signal Spectrum")
plot_freq_response(input_signal, base_sample_rate, "Input Signal Spectrum")
plt.show()
