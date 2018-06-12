readInts :: String -> [Int]
readInts s  = map read $ words s :: [Int]


deleteMin :: [Int] -> [Int]
deleteMin xs = [ x | x <- xs, x /= (minimum xs) && x /= (maximum xs) ]

main = do  
    putStrLn "Enter a value of a list through the spaces:"  
    name <- getLine  
    print $ deleteMin $ readInts name