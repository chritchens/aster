#! Sum example

(import std.io () io)
(import std.math (prod sum))

(defsig main (Fun IO IO))
(defun main io (io.println io (sum 1 2)))

(main io.stdIO)
