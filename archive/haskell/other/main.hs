main = do
 line1 <- getLine
 let num = read line1 :: Int

 line2 <- getLine
 let list = map read (words line2) :: [Int]

 print $ filter (\x -> x == list !! num) list
 
