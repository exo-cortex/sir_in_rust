#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

file_rk4_s = np.loadtxt("timeseries_s.txt")
file_rk4_i = np.loadtxt("timeseries_i.txt")
file_rk4_r = np.loadtxt("timeseries_r.txt")

plt.plot(file_rk4_s[:,0], file_rk4_s[:,1], "-", ms=1.5, linewidth=1.25, label="susceptible")
plt.plot(file_rk4_i[:,0], file_rk4_i[:,1], "-", ms=1.5, linewidth=1.25, label="infected")
plt.plot(file_rk4_r[:,0], file_rk4_r[:,1], "-", ms=1.5, linewidth=1.25, label="removed")

plt.legend()
plt.show()