{- видаляє останній елемент зі списку що співпадає з m-}
delLast m ys = if (last ys) == m then (init ys) else (delLast m (init ys)) ++ [last(ys)]

{- рекурсиво n раз видалити найбільший елемент -}
delNmax 0 xs = xs
delNmax n xs = delNmax (n - 1) (delLast (maximum xs) xs) 

main = print$ delNmax 3 [1,2,3,4,1,2,3,4]