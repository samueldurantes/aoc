module Solution where

import Data.List (groupBy, sort)

parseInput :: String -> [[Int]]
parseInput inp =
  let xs = groupBy (\_ y -> y /= "") (lines inp) in
  let ys = map (filter (/= "")) xs in
  map (map read) ys

main :: IO ()
main = do
  content <- readFile "input.txt"
  putStrLn $ "Part one: " ++ show (maximum $ sort $ map sum (parseInput content))
  putStrLn $ "Part two: " ++ show (sum $ take 3 $ reverse $ sort $ map sum (parseInput content))