module Main where

import Control.Monad (when)
import Data.Char (digitToInt)
import Utils

parseInput = (map . map) readInt . map words

solvePartOne = length . filter aux . parseInput
  where
    aux list = (isValidIncreasingSequence list) || (isValidDecreasingSequence list)

solvePartTwo = length . filter aux . parseInput
  where
    aux list = (max (maxPair $ longestIncreasingSubsequence (\x y -> isInRange (x, y) && x < y) list) (maxPair $ longestIncreasingSubsequence (\x y -> isInRange (x, y) && x > y) list)) > length list - 2

isInRange pair = absDiff pair >= 1 && absDiff pair <= 3

isValidSequence op list = foldl aux True (zip list (tail list))
  where
    aux acc pair = acc && isInRange pair && uncurry op pair

isValidIncreasingSequence = isValidSequence (<)

isValidDecreasingSequence = isValidSequence (>)

-- Subproblem definition:
-- X(i) is LIS starting at i for [i:n]
-- Recurrence relation:
-- X(i) = 1 + max(X(j), i+1<=j<=n if op(list[i], list[j]))
-- Topological order:
-- i decreasing
-- Base case:
-- X(n) = 0
-- Original problem:
-- X(0)
-- Time complexity:
-- O(n*n) 
-- Note that = O(nlogn) solution ex, but I did not want to implement binary search in Haskell :)
longestIncreasingSubsequence op = foldr aux []
  where
    aux val acc = (1 + impl op val acc, val) : acc
    impl op val acc = foldl (other op val) 0 acc
    other op ref acc (count, x) = if op ref x then max acc count else acc

maxPair = foldl aux 0
  where
    aux acc (x, _) = max acc x

main :: IO ()
main = do
  contents <- readInput
  print $ solvePartTwo contents
  -- print $ maxPair $ longestIncreasingSubsequence (<) [3, 2, 3, 4, 5]
  return ()
