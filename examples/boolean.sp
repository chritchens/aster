(module main (prod 
  #! An example on how booleans can be built using primitives

  (import std.io _ printf)

  (attrs True (prod asSize 1))
  (type True Prim)

  (attrs True (prod asSize 0))
  (type False Prim)

  (type Bool (Sum True False))

  (sig true True)
  (val true prim)

  (sig false False)
  (val false prim)

  (sig boolToUInt (Fun Bool UInt))
  (val boolToUInt (fun pred
    (case pred 
      (match true (fun p 1))
      (match false (fun p 0)))))

  (sig printBoolAsUInt (Fun (Prod IO Bool) IO))
  (val printBoolAsUInt (fun (prod pred io)
    (case pred
      (match true (fun t 
        (printf io "true as uint: {}\n" (boolToUInt t))))
      (match false (fun f 
        (printf io "false as uint: {}\n" (boolToUInt f)))))))

  (sig main (Fun IO IO))
  (val main (fun io (printBoolAsUInt (prod false io))))
))
