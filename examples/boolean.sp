#! An example on how booleans can be built using primitives.
(import std.io (prod printf stdIO))

(def type True Prim)
(def type False Prim)

(def type Bool (Sum True False))

(def sig true True)
(def prim true)

(def sig false False)
(def prim false)

(def sig boolToUInt (Fun Bool UInt))
(def fun boolToUInt pred
  (case pred 
    true (fun p 1)
    false (fun p 0)))

(def sig printBoolAsUInt (Fun IO Bool IO))
(def fun printBoolAsUInt io pred 
  (case pred 
    true  (fun t 
      (printf io "true as uint: {}\n" (boolToUInt t)))
    false (fun f 
      (printf io "false as uint: {}" (boolToUInt f)))))

(def sig main (Fun IO IO))
(def fun main io (printBoolAsUInt io false))

(main stdIO)
