#! Hello World example

(import std.io)
(import std.string)

(defsig main (Fun Empty IO))
(defun main e (println "Hello World!" e))

(main ())
