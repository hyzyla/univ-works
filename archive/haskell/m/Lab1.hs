
{-- отримати n-елементи списку --}
getN :: Int -> [Int] -> Int
getN n xs = xs !! n

{-- отримати індекси лише тих елементів, що дорівнюю n-тому --}
task :: Int -> [Int] -> [Int]
task n xs = [i | (i, x) <- zip [1..] xs, x == getN n xs]

main = do
    print $ (task 0 [1,2,3,4,1,1,1,1,2])