#! Sum example

(import std.io () io)
(import std.math (sum))

(defsig main (Fun Empty IO))
(defun main e (io.println (sum 1 2) e))

(main ())
