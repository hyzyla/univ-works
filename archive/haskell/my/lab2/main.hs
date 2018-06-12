readInts :: String -> [Int]
readInts s  = map read $ words s :: [Int]

{- Count occurence of the x in xs list of ints-}
countElem :: Int -> [Int] -> Int
countElem x xs = length $ filter (x==) xs

{- Filter only elements that occures twice in list -}
doublesElements :: [Int] -> [Int]
doublesElements xs = [x | x <- xs, countElem x xs == 2]

{-- Unique elements in the list -}
unique :: Eq a => [a] -> [a]
unique []       = []
unique (x : xs) = x : unique (filter (x /=) xs)

{- 
  Залишити у першоми списку елементи, що входять у другий список двічі.
-}
keepUseful :: [Int] -> [Int] -> [Int]
keepUseful xs ys = [x | x <- xs, y <- ys, x == y]




main = do  
    putStrLn "Enter a value of a list1 through the spaces:"  
    list1 <- getLine
    putStrLn "Do this again for a list 2:"   
    list2 <- getLine
    print $ keepUseful (readInts list1) (unique $ doublesElements $ readInts list2)