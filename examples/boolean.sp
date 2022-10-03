#! An example on how booleans can be built using primitives.
(import std.io (printf, stdIO))

(deftype True Prim)
(deftype False Prim)

(deftype Bool (Sum True False))

(defsig true True)
(defprim true)

(defsig false False)
(defprim false)

(defsig boolToUInt (Fun Bool UInt))
(defun boolToUInt pred
  (case pred 
    true 1 
    false 0))

(defsig printBoolAsUInt (Fun IO Bool IO))
(defun printBoolAsUInt io pred 
  (caseMap pred 
    true  (fun t 
      (printf io "true as uint: {}\n" (boolToUInt t)))
    false (fun f 
      (printf io "false as uint: {}" (boolToUInt f)))))

(defsig main (Fun IO IO))
(defun main io (printBoolAsUInt io false))

(main stdIO)
