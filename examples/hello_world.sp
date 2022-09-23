#! Hello World example

(import std.io)
(import std.string)

(defsig main (Fun IO IO))
(defun main io (println io "Hello World!"))

(main stdIO)
