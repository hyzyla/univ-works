
getPlato (a:[]) t r = r ++ [a]
getPlato (a:b:xs) t r
    | t == 'd'  && a > b = []
    | t == 'd'  && a < b = r ++ [a]
    | t == 'u'  && a < b = []
    | t == 'u'  && a > b = r ++ [a]
    | t == 'f'  && a /= b = r ++ [a] 
    | a == b = getPlato (b:xs) t (r ++ [a]) 

getFirstElems (a:b:xs) 
    | a < b = [a]
    | a > b = [a]
    | a == b = getPlato (a:b:xs) 'f' []

getElem :: (Int, Int, Int) -> [Int] -> [Int]
getElem (a, b, c) [] 
    | a >= b && b >  c = [c]
    | a <= b && b <  c = [c]
    | b > a && b >= c = [b,c]
    | b < a && b <= c = [b,c]
getElem (a, b, c) xs
    | b >  a && b > c  = [b]
    | b <  a && b < c  = [b]
    | b <  a && b == c = getPlato (b:c:xs) 'd' [] 
    | b >  a && b == c = getPlato (b:c:xs) 'u' []
    | b == c && b == a = []
    | otherwise = []

task :: [Int] -> [Int]
task (a:b:c:[]) = getElem(a,b,c) [] 
task (a:b:c:xs) = getElem(a,b,c) xs ++ task (b:c:xs)  

prog xs = getFirstElems xs ++ task xs

main = do 
    print $ prog [1,2,12,12,12,1,1,1,0,1,1,1,13]
