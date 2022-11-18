(module main (prod
  #! Sum example

  (import std.io _ _ io)
  (import std.math _ (prod +))

  (sig main (Fun IO IO))
  (val main (fun io 
    (io.println io (+ 1 2))))
))
