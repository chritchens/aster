(module main (prod 
  #! Hello World example

  (import std.io)
  (import std.string _ removeAll string)

  (def main (Fun IO IO))
  (def main (fun io (println io (string.removeAll '\'' "Hel'lo World!"))))
))
