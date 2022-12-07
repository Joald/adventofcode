let rec take n l = 
  match n, l with
    0, _ -> []
  | n, h :: t -> h :: take (n - 1) t
  | _ -> [];;
let rec uniq s = 
  match s with
    h :: t -> List.for_all (fun x -> x <> h) t && uniq t
  | [] -> true;;
let part1 () = 
  let f = open_in "input.txt" in
  let line = input_line f in 
  let res = String.fold_left (fun (i, acc, res) c ->
    (*Printf.printf "%b" res;*)
    if res then i, acc, res else
    let len = List.length acc in 
    if len < 14 
      then i + 1, c::acc, res
      else let newacc = (c::(take 13 acc))
        in if uniq newacc
        then i + 1, newacc, true
        else i + 1, newacc, false
(*    match acc with
      x :: y :: z :: zz :: _ ->
        Printf.printf "%c%c%c%c\n" x y z zz;
        Printf.printf "%b\n" (uniq [x;y;z;zz]); 
        let res = uniq [x;y;z;zz] in
        i + (if res then 0 else 1), [c;x;y;z], res
    | t -> i + 1, c :: t, res*)
    ) (0, [], false) line in 
  let final = match res with (i, _, _) -> i in
  Printf.printf "%s\n" (let _, v, _ = res in String.of_seq (List.to_seq v));
  Printf.printf "%d\n" final;;



part1 ()
