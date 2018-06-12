{- відстань задана умовою -}
distance x y z = abs (x - y) + abs (x - z)+ abs (z -y)

{- рекурсивно знаходить наменше значення за умовою у списку кортежів з трьох елементів -}
getMinValue [] n = n
getMinValue ((x, y, z):xs) n
   | (distance x y z) < n = getMinValue xs (distance z y z)
   | otherwise = getMinValue xs n

{- перевіряє умову чи будь-які з трьох елементів мають спільну координату, 
викорстовуєтсья для порівнювання елеменів самих з собою -}
hasSameIndex i j k = i == j || j == k || k == i

{- робить з одного списку список комбінацій всіх елементів, окрім самих з собою -}
getListOfTuples xs = [(x,y,z) |(i, x) <- zip [1..] xs, (j, y) <- zip [1..] xs, (k, z) <- zip [1..] xs, not $ hasSameIndex i j k ]

{- серед комбінацій списку вібирає лише ті щ дорівнюють мінімальній дистанції знайденної за допомогою getMinValue-}
task xs = [(x,y,z) | (x,y, z) <- (getListOfTuples xs), (distance z y z) == getMinValue (getListOfTuples xs) (maxBound :: Int)]

main = print  $ task [1,1,2,3]