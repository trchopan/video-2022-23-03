module Lib where


isMine :: Char -> Bool
isMine '*' = True
isMine _   = False


mineToStr :: Int -> String
mineToStr 0    = "."
mineToStr (-1) = "*"
mineToStr n    = show n


newtype Adjacent = Adjacent (Int, Int) -- (i, j)


checkMinesAdjacents :: Int -> Int -> (Int, Int) -> [Adjacent]
checkMinesAdjacents rows cols (i, j) = map Adjacent $ filter
  (\(x, y) -> x >= 0 && y >= 0 && x < cols && y < rows)
  [ (i - 1, j - 1)
  , (i    , j - 1)
  , (i + 1, j - 1)
  , --
    (i - 1, j)
  , (i + 1, j)
  , --
    (i - 1, j + 1)
  , (i    , j + 1)
  , (i + 1, j + 1)
  ]


genMineSweeper :: [[Char]] -> [[Int]]
genMineSweeper mines2d = zipWith
  (\j xs -> zipWith
    (\i c -> case isMine c of
      True -> (-1)
      False ->
        sum
          $ map (const 1) --
          $ filter isMine --
          $ map (\(Adjacent (i, j)) -> mines2d !! j !! i) adjacents
        where adjacents = checkMinesAdjacents rows cols (i, j)
    )
    [0 ..]
    xs
  )
  [0 ..]
  mines2d

 where
  rows = length mines2d
  cols = length $ head mines2d


-- divisible :: Integer -> Integer -> Bool
-- divisible n x = n `mod` x == 0


-- isPrime :: Integer -> Bool
-- isPrime n = isNothing (find (divisible n) [2 .. sqrtN])
--   where sqrtN = roundFloatInteger $ sqrt (fromInteger n)


-- nthPrime :: Int -> Integer
-- nthPrime nth = last $ take nth (filter isPrime [2 ..])


-- mkPrimes :: Integer -> [Integer]
-- mkPrimes maxN = if maxN < 2 then [] else filter isPrime [2 .. maxN + 1]
