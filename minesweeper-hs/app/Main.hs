module Main where
import           Lib                            ( genMineSweeper
                                                , mineToStr
                                                )


sampleMineSweeper :: [[Char]]
sampleMineSweeper =
  [ "·*·*·" --
  , "··*··"
  , "··*··"
  , "·····"
  ]

main :: IO ()
main = do
  putStrLn --
    $ foldl (\acc c -> acc ++ c ++ "\n") ""
    $ map (concatMap mineToStr)
    $ genMineSweeper sampleMineSweeper
