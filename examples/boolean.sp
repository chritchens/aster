#! An example on how booleans can be built using primitives.

(deftype True Prim)
(deftype False Prim)

(deftype Bool (Sum True False))

(defsig true True)
(defprim true)

(defsig false False)
(defprim false)
