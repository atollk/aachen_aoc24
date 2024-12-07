module Main where
import GHC.IO.FD
import Data.ByteString
import GHC.IO.Handle

read_file :: () -> String = do
  let list = []
  handle <- openFile "test.txt" ReadMode
  contents <- hGetContents handle
  let singlewords = words contents
      list = f singlewords
  hClose handle
  list

main :: IO ()
main = putStrLn "Hello, Haskell!"
