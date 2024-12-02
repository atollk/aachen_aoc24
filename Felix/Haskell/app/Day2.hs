module Main where

import Utils

maxPair :: [(Int, b)] -> Int
maxPair = foldl aux 0
  where
    aux acc = max acc . fst

isValidSequence :: (Ord b, Num b) => (b -> b -> Bool) -> [b] -> Bool
isValidSequence comp seq = foldl aux True (zip seq (tail seq))
  where
    aux acc pair = acc && isInRange pair && uncurry comp pair

isInRange :: (Ord a, Num a) => (a, a) -> Bool
isInRange pair = absDiff pair >= 1 && absDiff pair <= 3

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
longestIncreasingSubsequence :: (Foldable t, Num a1, Ord a1) => (a2 -> a2 -> Bool) -> t a2 -> [(a1, a2)]
longestIncreasingSubsequence pred = foldr getCurrLIS []
  where
    getCurrLIS curr acc = (1 + findPrevLIS pred curr acc, curr) : acc
    findPrevLIS pred curr = foldl (isValid pred curr) 0
    isValid pred ref acc (count, candidate) = if pred ref candidate then max acc count else acc

parseInput :: [String] -> [[Int]]
parseInput = map (map readInt . words)

solvePartOne :: [String] -> Int
solvePartOne = length . filter aux . parseInput
  where
    aux list = isValidSequence (<) list || isValidSequence (>) list

solvePartTwo :: [String] -> Int
solvePartTwo = length . filter isValidDampenedSequence . parseInput
  where
    isValidDampenedSequence list = max (maxPair $ longestIncreasingSubsequence (adjacentEval (<)) list) (maxPair $ longestIncreasingSubsequence (adjacentEval (>)) list) > length list - 2
    adjacentEval comp x y = isInRange (x, y) && comp x y

main :: IO ()
main = do
  contents <- readInput
  print $ solvePartOne contents
  print $ solvePartTwo contents
  return ()
