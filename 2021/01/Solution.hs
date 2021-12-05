module Solution where

pairs :: [Int] -> [(Int, Int)]
pairs [] = []
pairs xs = zip xs (tail xs)

f :: [Int] -> Int
f x = length (filter (/= False) (map (\y -> snd y > fst y) (pairs x)))

f' :: [Int] -> [Int]
f' [] = []
f' (x:xs) = x + sum (take 2 xs) : f' xs

main :: IO ()
main = do
  input <- readFile "input.txt"
  let list_input = map (\x -> read x :: Int) (words input)

  putStrLn $ "Part one: " ++ show (f list_input)
  putStrLn $ "Part two: " ++ show (f (f' list_input))
