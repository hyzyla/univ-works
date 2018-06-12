import numpy as np
import matplotlib.pyplot as plt


def find_local_maxima(data):
    res_y = []
    res_x = []
    for i, x in enumerate(data[1:-1], start=1):
        if data[i-1] < x > data[i + 1]:
            res_y.append(x)
            res_x.append(i)
    return res_x, res_y
        

def func(k, data):
    N = len(data)
    res =  1 / N
    a = 0
    b = 0
    for m, x in enumerate(data):
        a += x * np.cos(-2 * np.pi * k * m  / N) # (np.cos(-2 * np.pi * k * m  / N) ** 2 + np.sin(-2 * np.pi * k * m  / N) ** 2
        b += x * np.sin(-2 * np.pi * k * m  / N)

    return res * np.sqrt(a ** 2 + b ** 2)

if __name__ == "__main__":
    T = 5
    data = np.loadtxt('data/f1.txt', delimiter=' ')
    delta_f = 1 / T
    N = len(data)
    t = np.arange(N)
    r = np.array([func(k, data) for k in t])
    x, y = find_local_maxima(r)
    print(r)
    for res_x, res_y in zip(x, y):
        print(res_x, res_y)
    fig = plt.figure()
    ax = fig.add_subplot(111)
    ax.plot(t, r)
    ax.scatter(x, y)    
    plt.show()
