(module main (prod
  #! Hello World example

  (import std.io)
  (import std.string)

  (sig main (Fun IO IO))
  (val main (fun io (println io "Hello World!")))
))
