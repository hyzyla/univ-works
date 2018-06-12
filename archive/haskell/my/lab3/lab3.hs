{-  фунція перевіряє чи елемент є піком, і якщо так, то повертає список
    з коретежем, де перший елемент це є саме пікове значення, а другий позиція у списку
    яка вираховується за допомогою n, що позначає початкову довжину списку 
    мінус довжина поточного списку без оброблених елементів
-}
getPeak :: [Int] -> Int -> [(Int, Int)]
getPeak (a:b:c:xs) n 
    | a < b && b > c = [(b, n - length (c:xs))]
    | otherwise = []


{-  фунція проходитсья рекурсиво проходиться по елементах списку, 
    виклчаючи кожного разу перший елемент. 
    Параметр n позначає довжину початкового списку і використовується
    для знаходження інедксу піку
-}
task :: [Int] -> Int -> [(Int, Int)]
task (a:b:c:[]) n  -- випадок коли залишилися три останні елементи
    | a < b && b > c = [(b, n - 1)]   -- у випадок коли центральний елемент більший за свої сусідів це і є пік
    | b < c = [(c, n - 1)]  -- перевіряємо останній елемент чи є він піком
    | otherwise = [] 
task (a:b:c:xs) n = (getPeak (a:b:c:xs) n) ++ (task (b:c:xs) n)


prog :: [Int] -> [(Int, Int)]
prog ([]) = []
prog (a:[]) = [(a,1)]
prog (a:b:[]) 
    | a > b = [(a, 1)]
    | otherwise = [(b, 2)]
prog (a:b:xs) = (if a > b then [(a, 1)] else [])  -- перевіряємо чи перший елемент є піком
                ++ task (a:b:xs) (length (a:b:xs)) -- і додаємо всі інші піки

main = do 
    print $ prog [5,4,3,4,5,3,7]