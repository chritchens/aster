(module main (prod 
  #! An example on how booleans can be built using atomics

  (import std.io _ printf)

  (attrs True (prod asSize 1))
  (type True Atomic)

  (attrs True (prod asSize 0))
  (type False Atomic)

  (type Bool (Sum True False))

  (sig true True)
  (val true atomic)

  (sig false False)
  (val false atomic)

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
