module Main where

import Data.Bifunctor
import Data.List (sort)
import Data.Map (empty, findWithDefault, insert, intersectionWithKey)
import Utils

toPair :: [a] -> (a, a)
toPair [x, y] = (x, y)

mapPair :: (a -> b) -> (a, a) -> (b, b)
mapPair f = bimap f f

toLists :: ([a], [b]) -> (a, b) -> ([a], [b])
toLists acc (x, y) = bimap (x :) (y :) acc

parseInput :: [String] -> ([Int], [Int])
parseInput = foldl toLists ([], []) . map ((toPair . map readInt) . words)

solvePartOne :: [String] -> Int
solvePartOne = sum . map absDiff . uncurry zip . mapPair sort . parseInput

solvePartTwo :: [String] -> Int
solvePartTwo = sum . intersectMaps . mapPair getCounts  . parseInput
  where
    getCounts = foldl updateOrInsert empty
    updateOrInsert m val = insert val (1 + findWithDefault 0 val m) m
    intersectMaps (lhs, rhs) = intersectionWithKey keyValueProduct lhs rhs
    keyValueProduct k x y = k * x * y

main :: IO ()
main = do
  contents <- readInput
  print $ solvePartOne contents
  print $ solvePartTwo contents
  return ()
