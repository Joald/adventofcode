with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Ada.Containers; use Ada.Containers;
with Ada.Containers.Vectors;
with Ada.Containers.Ordered_Sets;
with Ada.Containers.Ordered_Maps;
with Ada.Text_IO; use Ada.Text_IO;

procedure Solution is
   package String_Vectors is new
     Ada.Containers.Vectors
       (Index_Type   => Natural,
        Element_Type => Unbounded_String);

   use String_Vectors;

   type Coord is record
      X : Integer; -- North/South
      Y : Integer; -- East/West
   end record;
   
   --  function Hash (XY : Coord) return Hash_Type is
   --  begin
   --     return Hash_Type (Long_Long_Integer(XY.X) + Long_Long_Integer(XY.Y));
   --  end;

   function "<" (Left : Coord; Right : Coord) return Boolean is
   begin
      if Left.X < Right.X then 
         return True;
      else 
         if Left.X > Right.X then
            return False;
         else
            return Left.Y < Right.Y;
         end if;
      end if;
   end "<";


   package C_Sets is new
     Ada.Containers.Ordered_Sets (Element_Type => Coord);
      --   , Hash => Hash
      --   , Equivalent_Elements => "=");

   use C_Sets;

   function Parse_Lines (Lines : Vector) return Set is
      S : Set;
      XY : Coord;
      P : constant String := "#";
      Idx : Natural;
      Cnt : Natural;
      Line : Unbounded_String;
   begin
      for I in Lines.First_Index .. Lines.Last_Index loop
         Line := Lines (I);
         Cnt := Ada.Strings.Unbounded.Count (Source => Line, Pattern => P);
         XY.X := I;
         Idx := 0;
         for I in 1 .. Cnt loop
            Idx := Index
            (Source  => Line,
               Pattern => P,
               From    => Idx + 1);
            XY.Y := Idx;
            S.Insert (XY);
         end loop;
      end loop;
      return S;
   end Parse_Lines;

   package C_Maps is new Ada.Containers.Ordered_Maps
      ( Key_Type => Coord
      , Element_Type => Coord);

   use C_Maps;

   Type Dir is (N, S, W, E);

   Type Decision is record
      Target : Coord;
      Moving : Boolean;
   end record;

   function Decide (Elf : Coord; Elfs : Set; Cur_Dir : Dir) return Decision is
      Type Dirs_Offs is array (0 .. 3) of Coord;
      Dirs : constant Dirs_Offs := ((-1, 0), (1, 0), (0, -1), (0, 1));
      Direction_I : Integer;
      Base_Off : Coord;
      All_Empty : Boolean := True;
   begin
      for X_Off in -1..1 loop
         for Y_Off in -1..1 loop
            if (X_Off /= 0 or Y_Off /= 0) and Elfs.Contains ((Elf.X + X_Off, Elf.Y + Y_Off)) then
               All_Empty := False;
            end if;
         end loop;
      end loop;

      if All_Empty then
         return (Elf, False);
      end if;

      for E_Off in 0..3 loop
         Direction_I := (Dir'Enum_Rep (Cur_Dir) + E_Off) mod 4;
         Base_Off := Dirs (Direction_I);
         All_Empty := True;
         for Side_Off in -1..1 loop
            if Base_Off.X = 0 then
               if Elfs.Contains ((Elf.X + Side_Off, Elf.Y + Base_Off.Y)) then
                  All_Empty := False;
               end if;
            else
               if Elfs.Contains ((Elf.X + Base_Off.X, Elf.Y + Side_Off)) then
                  All_Empty := False;
               end if;
            end if;
         end loop;
         if All_Empty then
            return ((Elf.X + Base_Off.X, Elf.Y + Base_Off.Y), True);
         end if;
      end loop;
      return (Elf, False);
   end Decide;
   
   procedure Round (Elfs : In Out Set; Cur_Dir : Dir) is
      So_Far : Set;
      Dupes : Set;
      Res : Decision;
      Want : Map;
      No_Move : Set;
   begin
      for Elf of Elfs loop
         Res := Decide (Elf, Elfs, Cur_Dir);
         if Res.Moving then
            if So_Far.Contains (Res.Target) then
               Dupes.Include (Res.Target);
            else
               So_Far.Include (Res.Target);
            end if;
            Want.Include (Elf, Res.Target);
         else 
            No_Move.Include (Elf);
         end if;
      end loop;

      Elfs.Clear;
      Elfs.Union (No_Move);

      for C in Want.Iterate loop
         if Dupes.Contains (Want (C)) then
            Elfs.Include (Key (C));
         else
            Elfs.Include (Want (C));
         end if;
      end loop;
   end Round;



   F : File_Type;
   Lines : Vector;
   BLine : Unbounded_String;
   Elfs : Set;
   Cur_Dir : Dir := N;
   Min_Pos : Coord := (Integer'Last, Integer'Last);
   Max_Pos : Coord := (Integer'First, Integer'First);
   Size : Integer;
   Part : constant Integer := 2;
   Part2Set : Set;
   Round_No : Integer := 0;
begin
   Open (F, In_File, "test.txt");
   
   while not End_Of_File (F) loop
      BLine := To_Unbounded_String (Get_Line (F));
      Lines.Append (BLine);
   end loop;
   Elfs := Parse_Lines(Lines);

   if Part = 1 then
      for Rnd in 1 .. 10 loop
         --  for Elf of Elfs loop
         --     Min_Pos.X := Integer'Min(Min_Pos.X, Elf.X);
         --     Min_Pos.Y := Integer'Min(Min_Pos.Y, Elf.Y);
         --     Max_Pos.X := Integer'Max(Max_Pos.X, Elf.X);
         --     Max_Pos.Y := Integer'Max(Max_Pos.Y, Elf.Y);
         --  end loop;

         --  for X in Min_Pos.X - 1 .. Max_Pos.X + 1 loop
         --     for Y in Min_Pos.Y - 1 .. Max_Pos.Y + 1 loop
         --        if Elfs.Contains((X, Y)) then
         --           Put('#');
         --        else
         --           Put('.');
         --        end if;
         --     end loop;
         --     Put_Line("");
         --  end loop;

         --  Min_Pos := (Integer'Last, Integer'Last);
         --  Max_Pos := (Integer'First, Integer'First);

         Round(Elfs, Cur_Dir);
         Cur_Dir := Dir'Enum_Val ((Dir'Enum_Rep (Cur_Dir) + 1) mod 4);
      end loop;

      for Elf of Elfs loop
         Min_Pos.X := Integer'Min(Min_Pos.X, Elf.X);
         Min_Pos.Y := Integer'Min(Min_Pos.Y, Elf.Y);
         Max_Pos.X := Integer'Max(Max_Pos.X, Elf.X);
         Max_Pos.Y := Integer'Max(Max_Pos.Y, Elf.Y);
      end loop;

      Size := (Max_Pos.X - Min_Pos.X + 1) * (Max_Pos.Y - Min_Pos.Y + 1);

      Put_line ("Result is " & Integer'Image (Size - Integer(Elfs.Length)) 
         & ", Size=" & Integer'Image (Size) & ", Elfs.Length=" & Integer'Image(Integer(Elfs.Length)));
   else 
      Part2Set.Union(Elfs);
      while True loop
         Round(Elfs, Cur_Dir);
         Cur_Dir := Dir'Enum_Val ((Dir'Enum_Rep (Cur_Dir) + 1) mod 4);
         Round_No := Round_No + 1;
         if Elfs = Part2Set then
            Put_Line (Integer'Image(Round_No) & " rounds passed.");
            return;
         end if;
         Part2Set.Clear;
         Part2Set.Union(Elfs);
      end loop;
   end if;
end Solution;

