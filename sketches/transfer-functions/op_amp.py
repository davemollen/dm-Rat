from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# //////////////////////////////
# PREDEFINED S-DOMAIN COEFFICIENTS

# Predefined distortion pot values
class DistortionPotValue(float, Enum):
  ZERO = 0.
  ONE = 1.
  TEN = 10.
  HUNDRED = 100.
  THOUSAND = 1000.
  TEN_THOUSAND = 10000.
  HUNDRED_THOUSAND = 100000.

# Change the distortion_pot_value to see the difference in the frequency response
distortion_pot_value = DistortionPotValue.ONE

# Use predefined s-domain coefficients
match distortion_pot_value:
  case DistortionPotValue.ZERO:
    num = [0, 2.78425E-7,0.0027423,1]
    den = [0, 2.72149E-7,0.0027354,1]
  case DistortionPotValue.ONE:
    num = [2.72149E-17,2.78426E-7,0.0027423001,1]
    den = [2.72149E-17,2.72149E-7,0.0027354001,1]
  case DistortionPotValue.TEN:
    num = [2.72149E-16, 3.34916E-7, 0.002804401, 1]
    den = [2.72149E-16, 2.72152E-7, 0.002735401, 1]
  case DistortionPotValue.HUNDRED:
    num = [2.72149E-15,8.99814E-7,0.00342541,1]
    den = [2.72149E-15,2.72176E-7,0.00273541,1]
  case DistortionPotValue.THOUSAND:
    num = [2.72149E-14,6.5488E-6, 0.0096355, 1]
    den = [2.72149E-14, 2.72423E-7, 0.0027355, 1]
  case DistortionPotValue.TEN_THOUSAND:
    num = [2.72149E-13, 0.00006, 0.0717364, 1]
    den = [2.72149E-13, 2.74884E-7, 0.0027364, 1]
  case DistortionPotValue.HUNDRED_THOUSAND:
    num = [2.72149E-12,0.00062,0.6927454,1]
    den = [2.72149E-12,2.99503E-7,0.0027454,1]

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig1 = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([-10, 70])

# ///////////////////////////////
# GENERATED S-DOMAIN COEFFICIENTS

def generate_s_domain_coefficients(distortion_pot_value):
  z2_b0 = max(distortion_pot_value, 1.)
  z2_a0 = z2_b0 * 1e-10

  z1_b0 = 2.72149e-7
  z1_b1 = 0.0027354
  z1_a0 = 6.27638e-9
  z1_a1 = 0.0000069

  a0 = z1_b0 * z2_a0
  a1 = z1_b0 + z1_b1 * z2_a0
  a2 = z1_b1 + z2_a0
  b0 = a0
  b1 = a1 + z1_a0 * z2_b0
  b2 = z1_a1 * z2_b0 + z1_b1 + z2_a0

  return (
    [b0, b1, b2, 1.],
    [a0, a1, a2, 1.],
  )

# Use s-domain coefficients derived from just the distortion_pot_value
num, den = generate_s_domain_coefficients(distortion_pot_value.value)
print('s-domain coefficients', (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print('z-domain coefficients', (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig2 = plt.figure(2)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([-10, 70])
plt.show()