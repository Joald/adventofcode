open System.IO


let testing = false

let testStr = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"

let flip f x = fun y -> f y x

let splitByComma (line : string) = line.Split ","
let toOption lineList = 
  match List.map int lineList with
  | [x; y; z] -> Some (x, y, z)
  | _ -> None

let cubes = 
  (if testing 
    then List.ofSeq <| testStr.Split "\n" 
    else List.ofSeq <| File.ReadLines "input.txt") |>
  List.filter (fun (line : string) -> line <> "") |>
  List.map (splitByComma >> List.ofSeq >> toOption) |>
  List.choose id
  
let hset = Set.ofList cubes

let neis (x, y, z) = 
  List.map (function | (xdelta, ydelta, zdelta) -> (x + xdelta, y + ydelta, z + zdelta))
    [(-1, 0, 0); (1, 0, 0); (0, -1, 0); (0, 1, 0); (0, 0, -1); (0, 0, 1)]

let surface cubes cset = 
  cubes |> 
  List.map (fun x -> 6 - (neis x |> List.filter (flip Set.contains cset) |> List.length)) |> 
  List.sum

let part1 = surface cubes hset

printfn $"{part1}"

(* part 1 first try - 4482 *)

let aggCubes f ini = 
  List.fold (fun (accx, accy, accz) (x, y, z) -> (f accx x, f accy y, f accz z)) (ini, ini, ini) cubes

let (minx, miny, minz) = aggCubes min 1000
let (maxx, maxy, maxz) = aggCubes max -1

let allCoords = [ for x in minx + 1 .. maxx - 1 do 
                    for y in miny + 1 .. maxy - 1 do
                      for z in minz + 1 .. maxz - 1 do 
                        if Set.contains (x, y, z) hset |> not then
                          yield (x, y, z)]

let rec checkCoord (neiAcc : (int * int * int) list) (vis : (int * int * int) Set) = (* call with neiAcc = [(x, y, z)]*)
  if List.isEmpty neiAcc 
    then true
    else if List.exists (fun (x, y, z) -> x = minx || x = maxx || y = miny || y = maxy || z = minz || z = maxz) neiAcc
      then false
      else 
        let newNei = neiAcc |> List.collect neis |> List.filter (flip Set.contains vis >> not) |> List.distinct
        in let newVis = Set.union (Set.ofList newNei) vis
           in checkCoord newNei newVis

let internalCubes = allCoords |> List.filter (fun xyz -> let res = checkCoord [xyz] (Set.add xyz hset) in printfn $"result for {xyz} is {res}"; res)

printfn "%A" internalCubes

let part2 = surface cubes (Set.union hset (Set.ofList internalCubes))

printfn $"{part2}"
(* part 2 first try 2576 *)