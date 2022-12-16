module Main
  ( main
  , showPQ
  )
  where

import Prelude

import Data.Array (snoc)
import Data.Array as Array
import Data.Filterable (filter)
import Data.Foldable (foldl)
import Data.Int (fromString)
import Data.List as List
import Data.Map (Map)
import Data.Map as Map
import Data.Maybe (Maybe(..))
import Data.Set (Set)
import Data.Set as Set
import Data.String as String
import Data.Tuple (Tuple(..))
import Effect (Effect)
import Effect.Console (log)
import Effect.Exception.Unsafe (unsafeThrow)
import Effect.Ref (Ref)
import Effect.Ref as Ref
import Effect.Unsafe (unsafePerformEffect)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)

testStr :: String
testStr = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""

type Valve = { name :: String, flowRate :: Int, neis :: Array String }
parseLine :: String -> Valve
parseLine line = 
  case String.split (String.Pattern "rate=") line of 
    [pref, rest] -> 
      let name = String.take 2 $ String.drop (String.length "Valve ") pref
      in case String.split (String.Pattern ";") rest of 
          [pref2, rest2] -> 
            let flowRate = maybeError "bad flow rate" $ fromString pref2
                neiStrs = Array.drop 5 $ String.split (String.Pattern " ") rest2
                neis = map (String.take 2) neiStrs
            in { name, flowRate, neis }
          _ -> unsafeThrow "bad input"
    _ -> unsafeThrow "bad input"

type Position = Tuple Int String

-- ((remaining, accrued), (flowRate, name)): turned
type PQKey = Tuple (Tuple Int Int) Position
type PQKey2 = Tuple (Tuple Int Int) (Tuple Position Position)
type PQVal = Set String
type Pq = Map PQKey PQVal
type Pq2 = Map PQKey2 PQVal

maybeError :: forall a. String -> Maybe a -> a
maybeError _ (Just a) = a
maybeError s Nothing = unsafeThrow s

showStrSet :: Set String -> String
showStrSet s = "{" <> String.joinWith ", " (Set.toUnfoldable s) <> "}"

showPQElem :: PQKey -> PQVal -> String
showPQElem (Tuple (Tuple remaining accrued) (Tuple _ name)) turned =
  "accrued " <> show accrued <> ", at " <> name <> ", " <> show remaining <> " remaining," <> showStrSet turned <> " turned;"
part1loop :: Map String Valve -> Pq -> Tuple Int Pq
part1loop vMap pq = --(PQueue (Cons (Tuple accrued (Tuple (Tuple remaining turned) name)) t)) =
  let 
    obj = maybeError "map error" $ Map.findMax pq 
    Tuple (Tuple remaining accrued) (Tuple _ name) = obj.key
    turned = obj.value
    -- _ = unsafePerformEffect $ log $ showPQElem obj.key obj.value
    popped = Map.delete obj.key pq
    valve = maybeError "bad valve" (Map.lookup name vMap)
    pushed :: Pq
    pushed = if remaining == 0 then popped else foldl (
      \acc nei ->
          let newVal = turned
              neiValve = maybeError "bad valve nei" $ Map.lookup nei vMap
              newKey = Tuple (Tuple (remaining - 1) accrued) $ Tuple neiValve.flowRate nei
            in Map.insert newKey newVal acc
      ) popped valve.neis
    pushed' = 
      if remaining > 0 && valve.flowRate > 0 && not (Set.member name turned)
        then 
          let newVal = Set.insert name turned
              newKey = Tuple (Tuple (remaining - 1) $ accrued + (remaining - 1) * valve.flowRate) $ Tuple valve.flowRate name
          in Map.insert newKey newVal pushed
        else pushed
  in Tuple accrued pushed'

showPQ :: Pq -> String
showPQ pq =
  let keyset = Map.keys pq
      keyArr = Set.toUnfoldable keyset :: Array PQKey
      valList = Map.values pq
      valArr = List.toUnfoldable valList :: Array PQVal
  in "PQ{" <> String.joinWith ", " (Array.zipWith showPQElem keyArr valArr) <> "}"

part1LoopTailRec :: Map String Valve -> Pq -> Int -> Effect Int
part1LoopTailRec vMap pq acc = if Map.isEmpty pq then pure acc else do
    -- log $ "processing loop for " <> showPQ pq
    let (Tuple new pq') = part1loop vMap pq
    -- log $ "processed to value " <> show new <> ", new pq = " <> show pq'
    part1LoopTailRec vMap pq' (max acc new)

part1 :: Array String -> Effect Int
part1 lines = 
  let valves = map parseLine $ filter (_ /= "") lines
      vMap = Map.fromFoldable $ flip Array.zip valves $ map (\v -> v.name) valves
      aaValve = maybeError "no AA valve" $ Map.lookup "AA" vMap
      pq = Map.singleton (Tuple (Tuple 30 0) $ Tuple aaValve.flowRate "AA") Set.empty
  in do
    -- log $ "vMap=" <> show vMap
    part1LoopTailRec vMap pq 0

data Action = Goto String | Turn 
derive instance compare :: Eq Action
derive instance ord :: Ord Action

part2loop :: Map String Valve -> Pq2 -> Tuple (Tuple Boolean Int) Pq2
part2loop vMap pq = --(PQueue (Cons (Tuple accrued (Tuple (Tuple remaining turned) name)) t)) =
  let 
    obj = maybeError "map error" $ Map.findMax pq 
    Tuple (Tuple remaining accrued) fullPos@(Tuple myFullPos@(Tuple _ myPos) eleFullPos@(Tuple _ elePos)) = obj.key
    turned = obj.value
    -- _ = unsafePerformEffect $ log $ showPQElem obj.key obj.value
    popped = Map.delete obj.key pq
    newRemaining = remaining - 1
    
    myValve = maybeError "bad valve" (Map.lookup myPos vMap)
    eleValve = maybeError "bad valve" (Map.lookup elePos vMap)
    myActions = Array.mapWithIndex (const Goto) myValve.neis
    eleActions = Array.mapWithIndex (const Goto) eleValve.neis
    myActions' = 
      if remaining > 0 && myValve.flowRate > 0 && not (Set.member myPos turned)
        then snoc myActions Turn
        else myActions
    eleActions' = 
      if remaining > 0 && eleValve.flowRate > 0 && not (Set.member elePos turned)
        then snoc eleActions Turn
        else eleActions
    actionPairs :: Array (Tuple Action Action)
    actionPairs = if remaining == 0 then [] else do 
      myAc <- myActions'
      eleAc <- eleActions'
      pure $ Tuple myAc eleAc

    makePos :: String -> Position
    makePos name = 
      let valve = maybeError "bad nei" $ Map.lookup name vMap
      in Tuple valve.flowRate name 

    pushed :: Pq2
    pushed = foldl (
      \pq2 acPair -> 
        case acPair of
          Tuple Turn Turn -> if myPos == elePos then pq2 else 
            let 
              newAccrued = accrued + newRemaining * (myValve.flowRate + eleValve.flowRate)
              newKey = Tuple (Tuple newRemaining newAccrued) fullPos
              newValue = Set.insert myPos $ Set.insert elePos turned
            in Map.insert newKey newValue pq2
          Tuple Turn (Goto newElepos) -> 
            let 
              newAccrued = accrued + newRemaining * myValve.flowRate
              newKey = Tuple (Tuple newRemaining newAccrued) (Tuple myFullPos $ makePos newElepos)
              newValue = Set.insert myPos turned
            in Map.insert newKey newValue pq2
          Tuple (Goto newMypos) Turn ->
            let 
              newAccrued = accrued + newRemaining * eleValve.flowRate
              newKey = Tuple (Tuple newRemaining newAccrued) (Tuple (makePos newMypos) eleFullPos)
              newValue = Set.insert elePos turned
            in Map.insert newKey newValue pq2
          Tuple (Goto newMypos) (Goto newElepos) ->
            let
              newFullPos = Tuple (makePos newMypos) (makePos newElepos) 
              newKey = Tuple (Tuple newRemaining accrued) newFullPos
            in Map.insert newKey turned pq2
    ) popped actionPairs
    _ = registerRem remaining
  in Tuple (Tuple (remaining == 0) accrued) pushed

registerRem :: Int -> Unit
registerRem remaining = unsafePerformEffect $ do
  mini <- Ref.read miniRem
  when (remaining < mini) $ do
    Ref.write remaining miniRem
    log $ show remaining

miniRem :: Ref Int
miniRem = unsafePerformEffect $ Ref.new 26
part2LoopTailRec :: Map String Valve -> Pq2 -> Int -> Effect Int
part2LoopTailRec vMap pq acc = if Map.isEmpty pq then pure acc else do
    -- log $ "processing loop for " <> showPQ pq
    let (Tuple (Tuple fin new) pq') = part2loop vMap pq
        newVal = max acc new
    -- log $ "processed to value " <> show new <> ", new pq = " <> show pq'
    if fin then pure newVal else part2LoopTailRec vMap pq' newVal


part2 :: Array String -> Effect Int
part2 lines = 
  let valves = map parseLine $ filter (_ /= "") lines
      vMap = Map.fromFoldable $ flip Array.zip valves $ map (\v -> v.name) valves
      aaValve = maybeError "no AA valve" $ Map.lookup "AA" vMap
      startPos = Tuple aaValve.flowRate "AA"
      pq = Map.singleton (Tuple (Tuple 26 0) $ Tuple startPos startPos) Set.empty
  in do
    -- log $ "vMap=" <> show vMap
    part2LoopTailRec vMap pq 0


testing :: Boolean
testing = false

main :: Effect Unit
main = do
  contents <- readTextFile UTF8 "input.txt"
  let lines = String.split (String.Pattern "\n") $ if testing then testStr else contents
  let part = 2
  log =<< show <$> (if part == 1 then part1 else part2) lines

-- part1 first try: 1711, 10s
-- part2 first try: 2416, 54m
-- part1 test: 3.5s
-- part2 test: 58.6s
-- brute-force: run Dijkstra on solution graph and terminate paths when they're over 26 (just like DiCaprio :P)
-- complexity is technically polynomial wrt graph size but being exponential in 26 makes it take a long time
-- runtime is just under an hour (TODO: measure)
