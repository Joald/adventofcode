import System.IO
import Data.Char

part1 :: [String] -> Int
part1 lines = sum $ map (priority . uncurry pick . halve) lines
        

halve :: String -> (String, String)
halve s = (take half s, drop half s)
    where 
        half = div (length s) 2
        
pick :: String -> String -> Char
pick (h:left) right = if elem h right then h else pick left right

priority :: Char -> Int
priority c 
    | ord c >= ord 'a' = ord c - ord 'a' + 1
    | otherwise = ord c - ord 'A' + 27

part2 :: [String] -> Int
part2 lines = sum $ map (priority . pick3) $ chunkList 3 lines

pick3 :: [String] -> Char
pick3 [h:s1, s2, s3] = if elem h s2 && elem h s3 then h else pick3 [s1, s2, s3]

chunkList :: Int -> [a] -> [[a]]
chunkList x [] = []
chunkList x l = take x l : chunkList x (drop x l)

main :: IO ()
main = do
    handle <- openFile "input.txt" ReadMode
    contents <- hGetContents handle
    print $ part2 $ lines contents
