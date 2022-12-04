module Solution where

import Data.List (nub, intersect)

getPriority :: Char -> Int
getPriority chr = f 1 ((++) ['a'..'z'] ['A'..'Z'])
  where
    f i (x : xs) =
      if x == chr
        then i
        else f (i + 1) xs

splitHalf :: [a] -> ([a], [a])
splitHalf xs = splitAt (div (length xs) 2) xs

group3 :: [a] -> [[a]]
group3 [] = []
group3 xs = take 3 xs : group3 (drop 3 xs)

main :: IO ()
main = do
  content <- readFile "input.txt"
  putStrLn $ "Part one: " ++ show (sum $ map (getPriority . head . (\(a, b) -> nub a `intersect` nub b) . splitHalf) (lines content))
  putStrLn $ "Part two: " ++ show (sum $ map ((getPriority . head) . foldl1 (intersect . nub)) (group3 (lines content)))