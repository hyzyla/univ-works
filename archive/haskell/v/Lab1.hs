{-- отримати n-елемент списку --}
getN :: Int -> [Int] -> Int
getN n xs = xs !! n

{-- отримати всі елементи, що не дорівнюю n-тому --}
task :: Int -> [Int] -> [Int]
task n xs = [x | x <- xs, x /= getN n xs]

main = do
    print $ (task 0 [1,2,3,4,1,1,1,1,2])
