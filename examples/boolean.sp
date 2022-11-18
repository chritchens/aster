(module main (prod 
  #! An example on how booleans can be built using primitives

  (import std.io _ printf)

  (def True (attrs (prod asSize 1)))
  (def True Prim)

  (def True (attrs (prod asSize 0)))
  (def False Prim)

  (def Bool (Sum True False))

  (def true True)
  (def true prim)

  (def false False)
  (def false prim)

  (def boolToUInt (Fun Bool UInt))
  (def boolToUInt (fun pred
    (case pred 
      (match true (fun p 1))
      (match false (fun p 0)))))

  (def printBoolAsUInt (Fun IO Bool IO))
  (def printBoolAsUInt (fun io pred 
    (case pred
      (match true (fun t 
        (printf io "true as uint: {}\n" (boolToUInt t))))
      (match false (fun f 
        (printf io "false as uint: {}" (boolToUInt f)))))))

  (def main (Fun IO IO))
  (def main (fun io (printBoolAsUInt io false)))
))
