module Utils where

import System.Environment
import Data.Char
readInput :: IO [String]
readInput = do
  path <- getArgs
  contents <- readFile $ head path
  return (lines contents)

printInput :: IO ()
printInput = do
  readInput >>= print

trim :: String -> String
trim = f . f
  where
    f = reverse . dropWhile isSpace

absDiff :: (Num a) => (a, a) -> a
absDiff pair = abs $ uncurry (-) pair

readInt :: String -> Int
readInt str = read str :: Int

