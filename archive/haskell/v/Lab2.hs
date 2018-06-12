{-- Скільки раз зустрічається елемент у списку --}
countElem :: Int -> [Int] -> Int
countElem x xs = length $ filter (x==) xs

{-- яка кількість максимальна кількість однакових елементів у списку --}
maxCount :: [Int] -> Int
maxCount xs = maximum [countElem i xs | i <- xs]

{-- вибириє лише ті елементи що зустрічалися в списку максимальну кільіксть разів --}
task :: [Int] -> [Int]
task xs = unique [x | x <- xs, countElem x xs == maxCount xs  ]

{-- унікальні елементи списку --}
unique :: Eq a => [a] -> [a]
unique []       = []
unique (x : xs) = x : unique (filter (x /=) xs)

main = do
    print $ task [1,2,3,4,5,1,1,1,2,2,2]