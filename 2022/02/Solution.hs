module Solution where

data RPS
  = Rock
  | Paper
  | Scissors
  deriving (Show)

f :: String -> RPS
f x =
  case x of
    "A" -> Rock
    "B" -> Paper
    "C" -> Scissors
    "X" -> Rock
    "Y" -> Paper
    "Z" -> Scissors

g :: [a] -> (a, a)
g [a, b] = (a, b)
g _ = error "Too large!"

h :: (RPS, RPS) -> Int
h x =
  case x of
    (Rock, Paper)        -> 6 + 2
    (Paper, Rock)        -> 0 + 1
    (Rock, Rock)         -> 3 + 1
    (Scissors, Paper)    -> 0 + 2
    (Paper, Scissors)    -> 6 + 3
    (Scissors, Scissors) -> 3 + 3
    (Scissors, Rock)     -> 6 + 1
    (Rock, Scissors)     -> 0 + 3
    (Paper, Paper)       -> 3 + 2

i :: (RPS, RPS) -> Int
i x =
  case x of
    (Rock, Paper)        -> 3 + 1
    (Paper, Rock)        -> 0 + 1
    (Rock, Rock)         -> 0 + 3
    (Scissors, Paper)    -> 3 + 3
    (Paper, Scissors)    -> 6 + 3
    (Scissors, Scissors) -> 6 + 1
    (Scissors, Rock)     -> 0 + 2
    (Rock, Scissors)     -> 6 + 2
    (Paper, Paper)       -> 3 + 2

main :: IO ()
main = do
  content <- readFile "input.txt"
  putStrLn $ "Part 1: "  ++ show (sum $ map (h . g . map f . words) (lines content))
  putStrLn $ "Part 2: "  ++ show (sum $ map (i . g . map f . words) (lines content))
