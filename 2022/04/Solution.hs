module Solution where

import Data.List (groupBy, intersect)

f :: [String] -> (Int, Int)
f [x, y] = (read x, read y)

parseInput :: String -> [(Int, Int)]
parseInput inp =
  let x0 = filter (/= ",") $ groupBy (\a b -> a /= ',' && b /= ',') inp in
  let x1 = map (filter (/= "-") . groupBy (\a b -> a /= '-' && b /= '-')) x0 in
  let x2 = map f x1 in
  x2

intersectAll :: [Int] -> [Int] -> Bool
intersectAll xs ys =
  let x0 = intersect xs ys in
  x0 == xs || x0 == ys

hasIntersect :: [Int] -> [Int] -> Bool
hasIntersect xs ys = not $ null $ intersect xs ys

main :: IO ()
main = do
  content <- readFile "input.txt"
  putStrLn $ "Part one: " ++ show (length $ filter (== True) $ map ((\[x, y] -> intersectAll x y) . map (uncurry enumFromTo) . parseInput) (lines content))
  putStrLn $ "Part two: " ++ show (length $ filter (== True) $ map ((\[x, y] -> hasIntersect x y) . map (uncurry enumFromTo) . parseInput) (lines content))