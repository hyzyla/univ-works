import numpy as np
from PIL import Image
np.random.seed(0)

def iszero(x):
    return np.all(np.isclose(x, 0))

def grevilles_method(X):
    # Метод Гревіля
    m, n = X.shape
    a0 = X[:, 0][np.newaxis, :] # перший cтовчик
    A = a0 if iszero(a0) else a0 / (a0 @ a0.T) 
    
    for k in range(1, n):
        
        Xk = X[:, :k] # перші k стопвців
        ak = X[:, k][:, np.newaxis] # k-й стовпець

        dk = A @ ak
        ck = ak - (Xk @ dk) 
        bk = (1 / (1 + dk.T @ dk)) * dk.T @ A if iszero(ck) else (1 / (ck.T @ ck)) * ck.T
            
        A = np.r_[A - (dk @ bk),  bk] 

    return A

def penrose_moore(X, eps=1e-10):
    # Метод Мура-Пенроуза
    m, n = X.shape
    err = np.inf
    ds = 1
    prev = X.T
    while err > eps:
        current = np.linalg.lstsq((X.T @ X + ds * np.eye(n)), X.T, rcond=None)[0]
        err = np.linalg.norm(current - prev)
        prev = current
        ds = ds / 2
    return current

if __name__ == "__main__":
    

    X = np.array(Image.open("data/x2.bmp"))
    Y = np.array(Image.open("data/y5.bmp")) 

    # Додаємо рядок знизу зображення
    X = np.r_[X, np.ones(X.shape[1])[np.newaxis, :]]

    # Матриця випадкових чисел
    V = np.random.rand(Y.shape[0], X.shape[0])
    m, n = X.shape
    
    # Метод Гревіля
    Xg = grevilles_method(X)
    Zg = np.eye(m) - X @ Xg
    Ag = Y @ Xg + V @ Zg
    Yg = (Ag @ X).astype("uint8")

    # Метод Мура-Пенроуза
    Xm = penrose_moore(X)
    Zm = np.eye(m) - X @ Xm
    Am = Y @ Xm + V @ Zm
    Ym = (Am @ X).astype("uint8")

    # Об'єднумо всі зображення в одне і виводимо на екран
    Y = np.c_[Y, Ym, Yg]
    im = Image.fromarray(Y, mode="L")
    im.show()

