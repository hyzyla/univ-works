{-- n-те значення послідовності Фібоначчі --}
fib :: Int -> Int
fib 0 = 0
fib 1 = 1 
fib n = fib (n-1) + fib (n-2)

{-- послідовність чисел Фібоначчі --}
fibSeq :: Int -> [Int]
fibSeq n = [x | x <- map fib [1..n]]

{-- перевіряє чи число належить послідовності Фібоначчі --}
inFibSeq :: Int -> Int -> Bool
inFibSeq x n 
    | elem x (fibSeq n)  = True 
    | x < (last (fibSeq n))  = False 
    | otherwise = inFibSeq x (n + 1)

{-- видаляє елементи, які стоять на позиціях, що належать послідовності Фібоначі --}
deleteElem :: [Int] -> [Int]
deleteElem xs = [x | (i, x) <- zip [1..] xs, not $ inFibSeq i 1 ]

main = do
    print $ deleteElem [1,2,3,4,5,6,7, 8, 9]